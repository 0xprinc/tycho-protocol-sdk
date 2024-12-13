// SPDX-License-Identifier: AGPL-3.0-or-later
pragma experimental ABIEncoderV2;
pragma solidity ^0.8.13;

import {ISwapAdapter} from "src/interfaces/ISwapAdapter.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from
    "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import {FluidDexReservesResolver} from "./Interfaces/FluidInterfaces.sol";
import {IFluidDexT1} from "./Interfaces/iDexT1.sol";

/// @title Fluid Dex Adapter
contract FluidAdapter is ISwapAdapter {
    using SafeERC20 for IERC20;

    uint256 constant PRECISE_UNIT = 10 ** 18;

    FluidDexReservesResolver public resolver;

    constructor(address resolver_) {
        resolver = FluidDexReservesResolver(resolver_);
    }

    /// @dev enable receive as this contract supports ETH
    receive() external payable {}

    modifier checkTokens(bytes32 poolId, address sellToken, address buyToken) {
        address poolAddress = resolver.getPoolAddress(uint256(poolId));
        (address token0, address token1) = resolver.getPoolTokens(poolAddress);
        require(sellToken != buyToken);
        require(
            (sellToken == token0 && buyToken == token1)
                || (sellToken == token1 && buyToken == token0)
        );
        _;
    }

    function getPriceAt(
        bytes32 poolId,
        address sellToken,
        address buyToken,
        uint256 specifiedAmount,
        OrderSide side
    ) internal view returns (Fraction memory price) {
        address poolAddress = resolver.getPoolAddress(uint256(poolId));
        (address token0, address token1) = resolver.getPoolTokens(poolAddress);

        if (OrderSide.Sell == side) {
            (bool success, bytes memory output) = address(resolver).staticcall(
                abi.encodeWithSignature(
                    "estimateSwapIn(address,bool,uint256,uint256)",
                    poolAddress,
                    sellToken == token0,
                    specifiedAmount,
                    0
                )
            );
            require(success, "Swap estimation failed");
            price.numerator = abi.decode(output, (uint256));
        } else {
            (bool success, bytes memory output) = address(resolver).staticcall(
                abi.encodeWithSignature(
                    "estimateSwapOut(address,bool,uint256,uint256)",
                    poolAddress,
                    sellToken == token0,
                    specifiedAmount,
                    type(uint256).max
                )
            );
            require(success, "Swap estimation failed");
            price.numerator = abi.decode(output, (uint256));
        }
        price.denominator = 1;
    }

    /// @inheritdoc ISwapAdapter
    function price(
        bytes32 poolId,
        address sellToken,
        address buyToken,
        uint256[] memory specifiedAmounts
    )
        external
        view
        override
        checkTokens(poolId, sellToken, buyToken)
        returns (Fraction[] memory prices)
    {
        prices = new Fraction[](specifiedAmounts.length);
        for (uint256 i; i < specifiedAmounts.length; i++) {
            prices[i] = getPriceAt(
                poolId, sellToken, buyToken, specifiedAmounts[i], OrderSide.Sell
            );
        }
        return prices;
    }

    /// @inheritdoc ISwapAdapter
    function swap(
        bytes32 poolId,
        address sellToken,
        address buyToken,
        OrderSide side,
        uint256 specifiedAmount
    )
        external
        payable
        override
        checkTokens(poolId, sellToken, buyToken)
        returns (Trade memory trade)
    {
        if (specifiedAmount == 0) {
            return trade;
        }
        uint256 gasBefore = gasleft();
        address poolAddress = resolver.getPoolAddress(uint256(poolId));
        IFluidDexT1 pool = IFluidDexT1(poolAddress);

        (address token0,) = resolver.getPoolTokens(poolAddress);

        if (side == OrderSide.Sell) {
            trade.calculatedAmount = pool.swapIn{value: msg.value}(
                sellToken == token0, specifiedAmount, 0, msg.sender
            );
        } else {
            trade.calculatedAmount = pool.swapOut{value: msg.value}(
                sellToken == token0,
                specifiedAmount,
                type(uint256).max,
                msg.sender
            );
        }

        trade.gasUsed = gasBefore - gasleft();
        trade.price =
            getPriceAt(poolId, sellToken, buyToken, specifiedAmount, side);

        return trade;
    }

    /// @inheritdoc ISwapAdapter
    function getLimits(bytes32 poolId, address sellToken, address buyToken)
        external
        view
        override
        returns (uint256[] memory limits)
    {
        limits = uint256[](2);
        address poolAddress = resolver.getPoolAddress(uint256(poolId));
        (address token0, address token1) = resolver.getPoolTokens(poolAddress);
        FluidDexReservesResolvers.PoolWithReserves memory reserves =
            resolver.getPoolReservesAdjusted(poolAddress);
        address token = token0 == sellToken ? token0 : token1;
        uint8 decimal = uint8(
            sellToken == 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE
                ? 18
                : IERC20(sellToken).decimals()
        );
        limits[0] = getMaxReserves(
            decimal,
            token0 == sellToken
                ? reserves.limits.withdrawableToken0
                : reserves.limits.withdrawableToken1,
            token0 == sellToken
                ? reserves.limits.borrowableToken0
                : reserves.limits.borrowableToken1,
            token0 == sellToken
                ? reserves.collateralReserves.token0RealReserves
                : reserves.collateralReserves.token1RealReserves,
            token0 == sellToken
                ? reserves.debtReserves.token0RealReserves
                : reserves.debtReserves.token1RealReserves
        );

        token = token0 == buyToken ? token0 : token1;
        uint8 decimal = uint8(
            buyToken == 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE
                ? 18
                : IERC20(buyToken).decimals()
        );
        limits[1] = getMaxReserves(
            decimal,
            token0 == buyToken
                ? reserves.limits.withdrawableToken0
                : reserves.limits.withdrawableToken1,
            token0 == buyToken
                ? reserves.limits.borrowableToken0
                : reserves.limits.borrowableToken1,
            token0 == buyToken
                ? reserves.collateralReserves.token0RealReserves
                : reserves.collateralReserves.token1RealReserves,
            token0 == buyToken
                ? reserves.debtReserves.token0RealReserves
                : reserves.debtReserves.token1RealReserves
        );
    }

    function getMaxReserves(
        uint8 decimals,
        FluidDexReservesResolver.TokenLimit memory withdrawableLimit,
        FluidDexReservesResolver.TokenLimit memory borrowableLimit,
        uint256 realColReserves,
        uint256 realDebtReserves
    ) internal pure returns (uint256) {
        // Calculate max limit reserves
        uint256 maxLimitReserves =
            borrowableLimit.expandsTo + withdrawableLimit.expandsTo;

        // If expandsTo values are the same, set maxLimitReserves to the single
        // value
        if (borrowableLimit.expandsTo == withdrawableLimit.expandsTo) {
            maxLimitReserves = borrowableLimit.expandsTo;
        }

        // Calculate real reserves with decimal adjustment
        uint256 maxRealReserves = realColReserves + realDebtReserves;

        if (decimals > 12) {
            maxRealReserves *= 10 ** (decimals - 12);
        } else if (decimals < 12) {
            maxRealReserves /= 10 ** (12 - decimals);
        }

        // Return the minimum of maxLimitReserves and maxRealReserves
        return maxRealReserves < maxLimitReserves
            ? maxRealReserves
            : maxLimitReserves;
    }

    /// @inheritdoc ISwapAdapter
    function getCapabilities(bytes32, address, address)
        external
        pure
        override
        returns (Capability[] memory capabilities)
    {
        capabilities = new Capability[](4);
        capabilities[0] = Capability.SellOrder;
        capabilities[1] = Capability.BuyOrder;
        capabilities[2] = Capability.PriceFunction;
        capabilities[2] = Capability.HardLimits;
    }

    /// @inheritdoc ISwapAdapter
    function getTokens(bytes32 poolId)
        external
        view
        override
        returns (address[] memory tokens)
    {
        tokens = new address[](2);
        address poolAddress = resolver.getPoolAddress(uint256(poolId));
        (tokens[0], tokens[1]) = resolver.getPoolTokens(poolAddress);
    }

    /// @inheritdoc ISwapAdapter
    function getPoolIds(uint256, uint256)
        external
        view
        override
        returns (bytes32[] memory ids)
    {
        ids = new bytes32[](resolver.getTotalPools());
        for (uint256 i = 0; i < ids.length; i++) {
            ids[i] = bytes32(abi.encode(i));
        }
    }
}
