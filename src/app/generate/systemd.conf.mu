[Unit]
Description={{description}}
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory={{root}}
ExecStart={{root}}/{{name}}
Restart=on-failure # or always, on-abort, etc

[Install]
WantedBy=multi-user.target
