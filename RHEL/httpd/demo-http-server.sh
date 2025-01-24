#!/bin/bash

# Update system
yum update -y

# Install HTTP server
yum install -y httpd
systemctl enable httpd
systemctl start httpd

# Add a dummy page
echo '<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Modern HTML Page</title>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap" rel="stylesheet">
    <style>
        body {
            font-family: "Roboto", sans-serif;
            margin: 0;
            padding: 0;
            background: #f9f9f9;
            color: #333;
        }
        header {
            background: linear-gradient(90deg, #6a11cb, #2575fc);
            color: #fff;
            padding: 20px;
            text-align: center;
        }
        header h1 {
            margin: 0;
            font-size: 2.5rem;
        }
        main {
            padding: 20px;
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        section {
            max-width: 800px;
            margin-bottom: 20px;
            background: #fff;
            padding: 15px;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        section h2 {
            margin-top: 0;
            color: #6a11cb;
        }
        ul {
            padding-left: 20px;
        }
        footer {
            background: #333;
            color: #fff;
            padding: 10px;
            text-align: center;
        }
        footer p {
            margin: 0;
        }
        @media (min-width: 768px) {
            main {
                flex-direction: row;
                flex-wrap: wrap;
                justify-content: center;
            }
            section {
                flex: 0 0 45%;
                margin: 10px;
            }
        }
    </style>
</head>
<body>
    <header>
        <h1>Welcome to this testing Page</h1>
    </header>
    <main>
        <section>
            <h2>About</h2>
            <p>If you are here, then Congrats! There is a working http server on your machine.</p>
        </section>
        <section>
            <h2>Features</h2>
            <ul>
                <li>HTTP server is installed on your system</li>
                <li>HTTP server is up and running</li>
                <li>Your firewall rules provide access to the proper port</li>
            </ul>
        </section>
    </main>
    <footer>
        <p>&copy; 2025 Used for verification. Built with ❤️</p>
    </footer>
</body>
</html>' > /var/www/html/index.html

