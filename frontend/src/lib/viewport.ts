export function viewport(node: HTMLElement, onIntersect: (isIntersecting: boolean) => void) {
	const observer = new IntersectionObserver(
		([entry]) => {
			if (entry.isIntersecting) {
				onIntersect(true);
			}
		},
		{ threshold: 0.5 }
	);

	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
		}
	};
}
