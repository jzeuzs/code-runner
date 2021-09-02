import fastify from 'fastify';
import form from 'fastify-formbody';
import tio from 'tio.js';
import logger from '@mgcrea/fastify-request-logger';
import prettifier from '@mgcrea/pino-pretty-compact';
import { loadRoutes } from '#root/util';
import sensible from 'fastify-sensible';
import helmet from 'fastify-helmet';
import compress from 'fastify-compress';

tio.setDefaultTimeout(10000);

const app = fastify({ disableRequestLogging: true, logger: { prettyPrint: true, prettifier } });

app.register(form);
app.register(logger);
app.register(sensible);
app.register(helmet);
app.register(compress);

await loadRoutes(app);

app.listen(process.env.PORT!, '0.0.0.0');
