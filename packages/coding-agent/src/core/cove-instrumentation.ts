import { appendFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";

type EventPayload = Record<string, unknown>;

export function emitCoveInstrumentationEvent(event: string, payload: EventPayload = {}): void {
	const outputPath = process.env.PI_COVE_INSTRUMENTATION_PATH;
	if (!outputPath) return;

	try {
		mkdirSync(dirname(outputPath), { recursive: true });
		appendFileSync(
			outputPath,
			`${JSON.stringify({
				timestamp: new Date().toISOString(),
				event,
				source: "pi-coding-agent",
				profile: process.env.PI_SOURCE_PROFILE,
				...payload,
			})}\n`,
		);
	} catch {
		// Instrumentation must not affect benchmark behavior.
	}
}

export function summarizeCoveMessages(messages: unknown[]): EventPayload {
	let chars = 0;
	const roles: Record<string, number> = {};

	for (const message of messages) {
		chars += safeJsonLength(message);
		const role = getRole(message);
		if (role) roles[role] = (roles[role] ?? 0) + 1;
	}

	return {
		count: messages.length,
		approxChars: chars,
		approxTokens: Math.ceil(chars / 4),
		roles,
	};
}

function safeJsonLength(value: unknown): number {
	try {
		return JSON.stringify(value)?.length ?? 0;
	} catch {
		return String(value).length;
	}
}

function getRole(value: unknown): string | undefined {
	if (!value || typeof value !== "object") return undefined;
	const role = (value as { role?: unknown }).role;
	return typeof role === "string" ? role : undefined;
}
