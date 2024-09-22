ARG FUNCTION_DIR="/function"

# Use the slim version of Python 3.11
FROM python:3.11-slim

# Include global arg in this stage of the build
ARG FUNCTION_DIR

# Install FreeCAD and any other system dependencies
# This will be cached unless there's a change in the apt-get command
RUN apt-get update && apt-get install -y freecad

# Copy function code and requirements file after installing FreeCAD
# This ensures that if only the source code changes, the FreeCAD installation is still cached
RUN mkdir -p ${FUNCTION_DIR}
COPY src/ ${FUNCTION_DIR}

# Install the function's dependencies
RUN pip install --target ${FUNCTION_DIR} -r ${FUNCTION_DIR}/requirements.txt

# Set working directory to function root directory
WORKDIR ${FUNCTION_DIR}

# Set runtime interface client as default command for the container runtime
ENTRYPOINT [ "/usr/local/bin/python", "-m", "awslambdaric" ]
# Pass the name of the function handler as an argument to the runtime
CMD [ "lambda_function.lambda_handler" ]