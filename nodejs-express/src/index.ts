import express from 'express';

const app = express();

app.get('/ping', (req, res) => {
    res.send({ message: "pong" })
});

app.listen(3000, () => {
   console.log('Express listening on port 3000');
});