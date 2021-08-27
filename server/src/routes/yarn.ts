import type { FastifyInstance } from 'fastify';
import { fetch, FetchResultTypes } from '@sapphire/fetch';
import { cutText } from '@sapphire/utilities';
import type { YarnPkg } from '#types/YarnPkg';
import { trimArray } from '#root/util';

const fetchYarn = async (name: string) => {
	try {
		return await fetch<YarnPkg.PackageJson>(`https://registry.yarnpkg.com/${name}`, FetchResultTypes.JSON);
	} catch {
		return null;
	}
};

const parseAuthor = (author?: YarnPkg.Author) => {
	if (!author) return undefined;

	const authorName = `**${author.name}**`;
	const authorUrl = author.name.startsWith('@')
		? encodeURI(author.url ?? `https://www.npmjs.com/org/${author.name.slice(1)}`)
		: encodeURI(author.url ?? `https://www.npmjs.com/~${author.name}`);

	return `[${authorName}](${authorUrl})`;
};

export default (app: FastifyInstance, _: any, done: () => void) => {
	app.get('/', async (req, reply) => {
		const { name } = req.query as Record<string, string>;
		const result = await fetchYarn(name);

		if (!result) return reply.status(404);

		const maintainers = result.maintainers.map((user) => `[${user.name}](${user.url ?? `https://www.npmjs.com/~${user.name}`})`);
		const latestVersion = result.versions[result['dist-tags'].latest];
		const dependencies = latestVersion.dependencies ? trimArray(Object.keys(latestVersion.dependencies)) : null;
		const author = parseAuthor(result.author);
		const dateCreated = result.time ? new Date(result.time.created).toLocaleString() : 'Unknown';
		const dateModified = result.time ? new Date(result.time.modified).toLocaleString() : 'Unknown';
		const description = cutText(result.description ?? '', 1000);
		const latestVersionNumber = result['dist-tags'].latest;
		const license = result.license || 'None';
		const mainFile = latestVersion.main || 'index.js';

		return {
			name: result.name,
			url: `https://yarnpkg.com/package/${name}`,
			description: cutText(
				[
					description,
					'',
					author ? `Author: ${author}` : undefined,
					`Maintainers: **${trimArray(maintainers).join(', ')}**`,
					`Version: **${latestVersionNumber}**`,
					`Entry File: **${mainFile}**`,
					`License: **${license}**`,
					`Created at: **${dateCreated}**`,
					`Modified at: **${dateModified}**`,
					'',
					'_Package Dependencies:_',
					dependencies?.length ? dependencies.join(', ') : 'No dependencies.'
				]
					.filter((i) => i !== undefined)
					.join('\n'),
				2000
			)
		};
	});

	done();
};
