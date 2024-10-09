#!/bin/bash

# Set variables
APP_NAME="dvpn-server"
APP_DIR="/opt/$APP_NAME"
SERVICE_FILE="/etc/systemd/system/$APP_NAME.service"
SCRIPT_PATH="server.js"  # Path to your Node.js script relative to this installer
NODE_PATH=$(which node)  # Get the path to the Node binary
NPM_PATH=$(which npm)    # Get the path to npm

# Create the application directory
echo "Creating application directory at $APP_DIR..."
sudo mkdir -p $APP_DIR

# Copy Node.js app files
echo "Copying Node.js app files..."
sudo cp $SCRIPT_PATH $APP_DIR

# Copy package.json file
if [ -f "package.json" ]; then
    echo "Copying package.json..."
    sudo cp package.json $APP_DIR
else
    echo "package.json not found. Skipping dependency installation."
fi

# Install Node.js dependencies
if [ -f "$APP_DIR/package.json" ]; then
    echo "Installing Node.js dependencies in $APP_DIR..."
    cd $APP_DIR
    sudo $NPM_PATH install --production
else
    echo "No package.json found, skipping npm install."
fi

# Create a systemd service file
echo "Creating systemd service file at $SERVICE_FILE..."
sudo bash -c "cat > $SERVICE_FILE" <<EOL
[Unit]
Description=Node.js App - $APP_NAME
After=network.target

[Service]
ExecStart=$NODE_PATH $APP_DIR/$(basename $SCRIPT_PATH)
WorkingDirectory=$APP_DIR
Restart=always
User=nobody
Group=nogroup
Environment=NODE_ENV=production
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=$APP_NAME
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOL

# Reload systemd to apply changes
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Start the service
echo "Starting $APP_NAME service..."
sudo systemctl start $APP_NAME

# Enable the service to start on boot
echo "Enabling $APP_NAME service to start on boot..."
sudo systemctl enable $APP_NAME

echo "Installation complete. Your Node.js app is running as a service."
