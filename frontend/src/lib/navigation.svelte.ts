import { replaceState } from '$app/navigation';
import { resolve } from '$app/paths';

export enum AppSlide {
	Title = 0,
	OverviewSlide = 1,
	MotivationSlide = 2,
	DaocSlide = 3,
	SourceSlide = 4,
	StatsSlide = 5,
	CurrentStateSlide = 6,
	Hero = 100,
	Intro = 101,
	Creation = 102,
	History = 103,
	Optimization = 104,
	Checkout = 105
}

export const SlideLabels: Record<AppSlide, string> = {
	[AppSlide.Title]: 'Title',
	[AppSlide.OverviewSlide]: 'Overview',
	[AppSlide.MotivationSlide]: 'Motivation',
	[AppSlide.DaocSlide]: 'The Problem',
	[AppSlide.SourceSlide]: 'The Problem: Items',
	[AppSlide.StatsSlide]: 'The Problem: Stats',
	[AppSlide.CurrentStateSlide]: 'Current State',
	[AppSlide.Hero]: 'Hero',
	[AppSlide.Intro]: 'Intro',
	[AppSlide.Creation]: 'Creation',
	[AppSlide.History]: 'History',
	[AppSlide.Optimization]: 'Optimization',
	[AppSlide.Checkout]: 'Checkout'
};

function getSlideFromHash(hash: string): AppSlide | null {
	const cleanHash = hash.replace('#', '');
	const entry = Object.entries(SlideLabels).find(([, value]) => value.toLowerCase() === cleanHash);
	return entry ? (parseInt(entry[0], 10) as AppSlide) : null;
}

export class NavigationEngine {
	activeSlide: AppSlide = $state(
		typeof window !== 'undefined'
			? (getSlideFromHash(window.location.hash) ?? AppSlide.Title)
			: AppSlide.Title
	);

	init() {
		if (typeof window !== 'undefined') {
			const slide = getSlideFromHash(window.location.hash);
			if (slide !== null) {
				this.activeSlide = slide;
				this.executeScroll(this.activeSlide, 'instant');
			}
		}
	}

	scrollTo(step: AppSlide) {
		this.activeSlide = step;
		this.updateUrl(step);
		this.executeScroll(step, 'smooth');
	}

	updateFromScroll(slide: AppSlide) {
		if (this.activeSlide !== slide) {
			this.activeSlide = slide;
			this.updateUrl(slide);
		}
	}

	private updateUrl(slide: AppSlide) {
		if (typeof window !== 'undefined') {
			const hash = SlideLabels[slide].toLowerCase();
			const search = window.location.search;

			replaceState(resolve(`/${search}#${hash}` as '/'), {});
		}
	}

	private executeScroll(slide: AppSlide, behavior: ScrollBehavior) {
		const element = document.getElementById(`slide-${slide}`);
		if (element) {
			element.scrollIntoView({ behavior });
		}
	}
}

export const nav = new NavigationEngine();
