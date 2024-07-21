const fsPromise = require('fs').promises;
const nenoLab = require('neon-lab');
const path = require('path');

(async function () {
    let rootPath = '/Users/liyaya/Downloads/';
    const filenames = [
        'MicrosoftEdge-119.0.2151.72.pkg',
        'DingTalk_v7.1.10.14_33241691_universal.dmg',
        'TencentMeeting_0300000000_3.21.10.454.publish.arm64.officialwebsite.dmg',
        'mysql-8.0.36-macos14-arm64.tar.gz',
    ].map((filename) => path.join(rootPath, filename));

    for (const filename of filenames) {
        const stat = await fsPromise.stat(filename);
        const startMs = Date.now();
        const res = nenoLab.test01(filename);
        const endMs = Date.now();
        const speed = ((stat.size / 1024 / 1024) * 1000) / (endMs - startMs);
        console.log(
            `file size: ${stat.size} bytes, time used: ${
                Date.now() - startMs
            }ms speed: ${speed.toFixed(2)} MB/s, res: ${res}`
        );
    }
})();
