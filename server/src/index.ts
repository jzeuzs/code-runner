import fastify from 'fastify';
import piston from 'piston-client';
import form from 'fastify-formbody';
import tio from 'tio.js';
import logger from '@mgcrea/fastify-request-logger';
import prettifier from '@mgcrea/pino-pretty-compact';

tio.setDefaultTimeout(10000);

const client = piston({ server: 'https://emkc.org' });
const app = fastify({ disableRequestLogging: true, logger: { prettyPrint: true, prettifier } });

app.register(form);
app.register(logger);

app.post('/', async (req, _reply) => {
	const { body } = req as any;
	let { lang, code } = body;

	switch (lang) {
		case 'py':
		case 'py3':
		case 'python3':
			lang = 'python';
			break;
		case 'sh':
			lang = 'bash';
			break;
		case 'bf':
			lang = 'brainfuck';
			break;
		case 'clj':
			lang = 'clojure';
			break;
		case 'cob':
			lang = 'cobol';
			break;
		case 'coffee':
			lang = 'coffeescript';
			break;
		case 'cr':
			lang = 'crystal';
			break;
		case 'cs':
		case 'csharp':
			lang = 'dotnet';
			break;
		case 'exs':
			lang = 'elixir';
			break;
		case 'el':
		case 'elisp':
			lang = 'emacs';
			break;
		case 'erl':
		case 'escript':
			lang = 'erlang';
			break;
		case 'gcc':
			lang = 'c';
			break;
		case 'cpp':
		case 'g++':
			lang = 'c++';
			break;
		case 'gdc':
			lang = 'd';
			break;
		case 'f90':
			lang = 'fortran';
			break;
		case 'golang':
			lang = 'go';
			break;
		case 'gvy':
			lang = 'groovy';
			break;
		case 'hs':
			lang = 'haskell';
			break;
		case 'jl':
			lang = 'julia';
			break;
		case 'kt':
			lang = 'kotlin';
			break;
		case 'cl':
		case 'sbcl':
		case 'commonlisp':
			lang = 'lisp';
			break;
		case 'lol':
		case 'lci':
			lang = 'lolcode';
			break;
		case 'csharp':
		case 'cs':
			lang = 'mono';
			break;
		case 'asm':
		case 'nasm32':
			lang = 'nasm';
			break;
		case 'asm64':
			lang = 'nasm64';
			break;
		case 'js':
		case 'node-javascript':
		case 'node-js':
			lang = 'javascript';
			break;
		case 'ml':
			lang = 'ocaml';
			break;
		case 'm':
		case 'matlab':
			lang = 'octave';
			break;
		case 'usable':
		case '05AB1E':
			lang = 'osabie';
			break;
		case 'freepascal':
		case 'pp':
		case 'pas':
			lang = 'pascal';
			break;
		case 'pl':
			lang = 'perl';
			break;
		case 'php8':
		case 'html':
			lang = 'php';
			break;
		case 'pony':
		case 'ponyc':
			lang = 'ponylang';
			break;
		case 'plg':
			lang = 'prolog';
			break;
		case 'py2':
			lang = 'python2';
			break;
		case 'rakudo':
		case 'per16':
		case 'p6':
		case 'p16':
			lang = 'raku';
			break;
		case 'rock':
		case 'rocky':
			lang = 'rockstar';
			break;
		case 'rb':
		case 'ruby3':
			lang = 'ruby';
			break;
		case 'rs':
			lang = 'rust';
			break;
		case 'sc':
			lang = 'scala';
			break;
		case 'ts':
		case 'node-ts':
		case 'tsc':
			lang = 'typescript';
			break;
		case 'v':
			lang = 'vlang';
			break;
		case 'yeethon3':
			lang = 'yeethon';
			break;
		case 'idris': {
			lang = 'idris';
			const res = await tio(code, 'idris');

			return {
				language: res.language,
				output: res.output,
				stderr: res.exitCode !== 0 ? res.output : ''
			};
		}
		case 'mamba':
		case 'mb': {
			lang = 'mamba';
			const res = await tio(code, 'mamba');

			return {
				language: res.language,
				output: res.output,
				stderr: res.exitCode !== 0 ? res.output : ''
			};
		}
		case 'sql':
		case 'sqlite': {
			lang = 'sqlite';
			const res = await tio(code, 'sqlite');

			return {
				language: res.language,
				output: res.output,
				stderr: res.exitCode !== 0 ? res.output : ''
			};
		}
		case 'agda': {
			lang = 'agda';
			const res = await tio(code, 'agda');

			return {
				language: res.language,
				output: res.output,
				stderr: res.exitCode !== 0 ? res.output : ''
			};
		}
	}
	console.log(lang)

	const res = await client.execute(lang, code);

	console.log(res);

	return {
		language: res.language,
		output: res.run.output,
		stderr: res.run.stderr
	};
});

app.listen(3000, '0.0.0.0');
