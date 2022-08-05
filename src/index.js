let fs = require('fs');
if (fs.existsSync("package_backup.json")) {
    fs.copyFileSync('package_backup.json', 'package.json');
}
if (fs.existsSync("index_backup.html")) {
    fs.copyFileSync('index_backup.html', 'index.html');
}
