

// HTML directory template
pub const TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dusk</title>
    <style>
        body {
            font-family: microsoft yahei, "Helvetica Neue", "pingfang sc";
            padding: 0 24px 0;
        }
        h1 {
            font-weight: normal;
        }
        ul{
            padding-left: 0;
        }
        li {
            list-style-type: none;
            line-height: 20px;
            word-wrap: break-word;
            margin-top: 6px;
        }
    </style>
</head>
<body>
    <h1>Index of {title}</h1>
    <ul>
        <li>
            <a href="../">../</a>
        </li>
        {files}
    </ul>
</body>
</html>"#;


