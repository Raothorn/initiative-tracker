import express from 'express';
import { createServer } from 'node:http';
import { Monster } from 'share';

const app = express();
const server = createServer(app);

server.listen(3000, () => {
    let m: Monster = { name: "goblin" }
    console.log(m)
    console.log('server running at http://localhost:3000')
});

