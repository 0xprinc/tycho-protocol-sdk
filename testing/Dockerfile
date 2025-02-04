# Use an official Python runtime as a parent image
FROM --platform=linux/amd64 continuumio/miniconda3:24.4.0-0

# Set the working directory in the container to /app
WORKDIR /app

# Install Foundry
RUN apt-get update && apt-get install -y curl
RUN curl -L https://foundry.paradigm.xyz | bash
RUN /bin/bash -c "source $HOME/.bashrc && $HOME/.foundry/bin/foundryup"
#
# Add Foundry to PATH
ENV PATH /root/.foundry/bin:$PATH

# Add current directory code to /app in container
ADD . /app/testing

RUN chmod +x /app/testing/tycho-indexer

# Create a new conda environment and install pip
RUN conda create -n myenv pip python=3.9

# Install any needed packages specified in requirements.txt
RUN echo "source activate myenv" >~/.bashrc
ENV PATH /opt/conda/envs/myenv/bin:$PATH

RUN apt-get update \
    && apt-get -y install libpq-dev gcc \
    && pip install psycopg2 \
    && apt-get clean

ARG PIP_INDEX_URL
RUN /bin/bash -c "source activate myenv && cd testing && pip install --no-cache-dir -r requirements.txt && cd -"

# Make port 80 available to the world outside this container
EXPOSE 80

# Install the substreams cli
RUN wget -c https://github.com/streamingfast/substreams/releases/download/v1.8.0/substreams_linux_x86_64.tar.gz -O - | tar xzf - substreams
RUN mv substreams /usr/local/bin/substreams && chmod +x /usr/local/bin/substreams

# Run the command to start your application
CMD ["python", "testing/src/runner/cli.py", "--package", "my_substream", "--with_binary_logs", "--db_url", "postgres://postgres:mypassword@db:5432"]