class LayoutConfig {
	// Default to right side, can be switched dynamically to 'left'
	side = $state<'left' | 'right'>('right');

	toggleSide() {
		this.side = this.side === 'right' ? 'left' : 'right';
	}
}

export const layoutConfig = new LayoutConfig();
