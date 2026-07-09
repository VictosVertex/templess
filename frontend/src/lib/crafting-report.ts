export type CraftingReportGem = {
	id: number;
	label: string;
	value: number;
	tier: number;
};

export type CraftingReportEntry = {
	slotName: string;
	itemName: string;
	gems: CraftingReportGem[];
};

function sanitizeFilenamePart(value: string): string {
	return (
		value
			.toLowerCase()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/^-+|-+$/g, '') || 'template'
	);
}

function createCraftingReportContent(templateName: string, entries: CraftingReportEntry[]): string {
	const lines = [`Crafting Report: ${templateName}`, ''];

	if (entries.length === 0) {
		lines.push('No crafted items are currently equipped.');
		return lines.join('\n');
	}

	for (const entry of entries) {
		lines.push(`${entry.slotName}: ${entry.itemName}`);

		if (entry.gems.length === 0) {
			lines.push('  Gems: none assigned');
			lines.push('');
			continue;
		}

		for (const gem of entry.gems) {
			lines.push(`  - ${gem.value} ${gem.label} (tier ${gem.tier + 1}, gem id ${gem.id})`);
		}

		lines.push('');
	}

	return lines.join('\n');
}

export function downloadCraftingReport(templateName: string, entries: CraftingReportEntry[]) {
	const content = createCraftingReportContent(templateName, entries);
	const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');

	link.href = url;
	link.download = `${sanitizeFilenamePart(templateName)}-crafting-report.txt`;
	link.click();

	URL.revokeObjectURL(url);
}
