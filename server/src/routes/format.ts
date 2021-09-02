import type { FastifyInstance } from 'fastify';
import { fetch, FetchResultTypes } from '@sapphire/fetch';
import redis from '#root/db';
import { Seconds } from '#root/util';

// @ts-expect-error Doesn't work with default import
import * as toDataUrl from 'buffer-to-data-url';

export default (app: FastifyInstance, _: any, done: () => void) => {
	app.post('/', async (req, _reply) => {
		const { code } = req.body as Record<string, string>;
		const cached = await redis.getBuffer(`carbon-${code}`).catch(() => null);

		if (cached) {
			const url = await toDataUrl('image/png', cached);

			return { url };
		}

		const img = await fetch(
			'https://carbonara.vercel.app/api/cook',
			{
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					code,
					paddingVertical: '11px',
					paddingHorizontal: '14px',
					backgroundColor: 'rgba(74,74,74,1)',
					dropShadow: true,
					dropShadowOffsetY: '51px',
					dropShadowBlurRadius: '36px',
					theme: 'one-dark',
					windowTheme: 'none',
					language: 'auto',
					fontFamily: 'Fira Code',
					fontSize: '14px',
					lineHeight: '136%',
					windowControls: true,
					widthAdjustment: true,
					lineNumbers: false,
					firstLineNumber: 1,
					exportSize: '2x',
					watermark: false
				}),
				method: 'POST'
			},
			FetchResultTypes.Buffer
		);

		await redis.setex(`carbon-${code}`, Seconds.MONTH, img);

		const url = await toDataUrl('image/png', img);

		return { url };
	});

	done();
};
