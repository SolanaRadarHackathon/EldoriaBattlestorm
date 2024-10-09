
# Use the official lightweight Python image.
# https://hub.docker.com/_/python
FROM python:3.9-slim

LABEL authors="Eldahalas"

# Allow statements and log messages to immediately appear in the Knative logs
ENV PYTHONUNBUFFERED True

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Install any dependencies specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Expose port 5000 to allow communication to/from the Flask API server
EXPOSE 5000

# Command to run the Flask API server
CMD ["python", "app.py"]