import Fastify from 'fastify';

const fastify = Fastify({
  logger: false
});

fastify.get('/ping', (req, reply) => {
  reply.send({ message: 'pong' })
});

fastify.listen({ port: 3000 }, (err, address) => {
  if (err) {
    throw err
  }

  console.log('Fastify listening on port 3000');
});