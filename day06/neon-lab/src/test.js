const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

let content = fs.readFileSync(path.join(__dirname, '../cargo.log'), 'utf-8');
let res = crypto.Hash('sha256').update(content).digest('hex');
console.log(res);
let b = '68653983fb289c8916fa83d459cfccdcacd373041ab0cd4f17b77754c1402842';
