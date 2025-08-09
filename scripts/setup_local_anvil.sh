#!/usr/bin/env bash
# Local development helper for Anvil setup
set -e

# Check if running as root for mount operations
if [ "$EUID" -ne 0 ]; then
  echo "Please run with sudo for tmpfs mount operations"
  exit 1
fi

# Create tmpfs mount directory if it doesn't exist
mkdir -p /mnt/anvil-fork

# Check if already mounted
if mount | grep -q "/mnt/anvil-fork"; then
  echo "✅ /mnt/anvil-fork is already mounted as tmpfs"
else
  # Mount as tmpfs with 512MB size
  mount -t tmpfs -o size=512m tmpfs /mnt/anvil-fork
  echo "✅ Mounted /mnt/anvil-fork as 512MB tmpfs"
fi

# Add to fstab if not already there
if ! grep -q "/mnt/anvil-fork" /etc/fstab; then
  echo "tmpfs /mnt/anvil-fork tmpfs rw,size=512m 0 0" >> /etc/fstab
  echo "✅ Added tmpfs mount to /etc/fstab for persistence across reboots"
fi

echo "✅ Anvil tmpfs setup complete"
echo "Run 'df -h /mnt/anvil-fork' to verify the tmpfs mount"
