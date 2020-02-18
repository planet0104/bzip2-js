# bzip2-js

compile [bzip2-rs](https://github.com/alexcrichton/bzip2-rs) to javascript.

## use in HTML

```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<script src="bzip2.js"></script>
</head>
<body>
    <script>
        function stringToBytes(str) {
            var array = new Uint8Array(str.length);
            for (var i = 0, l = str.length; i < l; i++) {
                array[i] = str.charCodeAt(i);
            }
            return array;
        }

        var data = "hello!hello!hello!";
        var compressed = Bzip2.compress(stringToBytes(data));
        console.log("compressed=", compressed);
        document.body.append("compressed="+Bzip2.decompressUtf8String(compressed));
    </script>
</body>
</html>
```

## use in miniprogram

```javascript
    var Bzip2 = require("./bzip2.js");

    let input = "hello!hello!hello!";

    let result = Bzip2.compress(new Uint8Array(stringToBytes(input)));

    console.log('compressed:', result);

    console.log('decompressed:', Bzip2.decompressUtf8String(result));
```