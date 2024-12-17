// SPDX-License-Identifier: AGPL-3.0-or-later
pragma solidity ^0.8.13;

import {AdapterTest} from "./AdapterTest.sol";
import "forge-std/console.sol";
import {IERC20} from "lib/forge-std/src/interfaces/IERC20.sol";
import {FluidAdapter} from "src/fluid/FluidAdapter.sol";
import {ISwapAdapterTypes} from "src/interfaces/ISwapAdapterTypes.sol";
import {FractionMath} from "src/libraries/FractionMath.sol";
import {FluidDexReservesResolver} from "src/fluid/Interfaces/FluidInterfaces.sol";
import {Structs} from "src/fluid/Interfaces/structs.sol";
import {IFluidDexT1} from "src/fluid/Interfaces/iDexT1.sol";

contract FluidAdapterTest is AdapterTest {
    using FractionMath for Fraction;

    FluidAdapter adapter;
    FluidDexReservesResolver resolver;

    function setUp() public {

        uint256 mainnetFork = vm.createFork("https://eth.llamarpc.com", 21422012);
        vm.selectFork(mainnetFork);

        // Create a mock resolver
        resolver = FluidDexReservesResolver(0x45f4Ad57e300DA55C33DEa579A40FCeE000d7B94);
        
        // Deploy FluidAdapter with mock resolver
        adapter = new FluidAdapter(address(resolver));

        bytes32 poolId1 = bytes32(abi.encode(1)); // wstETH/Eth
        bytes32 poolId2 = bytes32(abi.encode(2)); // USDC/USDT

        address pool1Address = resolver.getPoolAddress(uint256(poolId1));
        address pool2Address = resolver.getPoolAddress(uint256(poolId2));
        (address pool1Token0, address pool1Token1) = resolver.getPoolTokens(pool1Address);
        (address pool2Token0, address pool2Token1) = resolver.getPoolTokens(pool2Address);
    }

    function test_price() public {}
    function test_swap() public {}
    function test_getLimits() public {}
    function test_getCapabilities() public {}
    function test_getTokens() public {}
    function test_poolIds() public {}

}