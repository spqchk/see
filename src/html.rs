

// HTML directory template
pub const TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index of {title}</title>
    <style>
        body {
            font-family: "pingfang sc", "microsoft yahei", "Helvetica Neue";
            padding: 0 24px 0;
        }
        h1 {
            font-weight: normal;
            word-wrap: break-word;
        }
        main{
            display: grid;
            grid-template-columns: {main};
        }
        a:first-child{
            grid-column: {first};
        }
        a, time, span{
            line-height: 20px;
            word-wrap: break-word;
            margin-top: 6px;
        }
        time, span{
            padding-left: 20px;
        }
    </style>
</head>
<body>
    <h1>Index of {title}</h1>
    <main>
        <a href="../">../</a>
        {files}
    </main>
</body>
</html>"#;


