const crypto = require('crypto');
const fsPromise = require('fs/promises');
const A = require('day05_4');
async function digestFile(filename) {
    const hasher = crypto.createHash('sha256');

    const fd = await fsPromise.open(filename, 'r');
    const buffer = Buffer.alloc(1024 * 64);

    for (;;) {
        const { bytesRead } = await fd.read(buffer, 0, buffer.length, null);

        if (bytesRead === 0) break;
        hasher.update(buffer.subarray(0, bytesRead));
        await fd.close();
        return hasher.digest('hex');
    }
}
digestFile('./package.json').then(console.log);
