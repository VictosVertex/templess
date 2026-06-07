<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { nav, AppSlide } from '$lib/navigation.svelte';

	import Header from '$lib/components/Header.svelte';
	import NavigationBar from '$lib/components/NavigationBar.svelte';
	import Slide from '$lib/components/Slide.svelte';
	import Hero from '$lib/slides/Hero.svelte';
	import TemplateCreation from '$lib/slides/TemplateCreation.svelte';
	import TemplateHistory from '$lib/slides/TemplateHistory.svelte';
	import Optimization from '$lib/slides/optimization/Optimization.svelte';
	import { TemplateBuilder } from '$lib/template-builder.svelte.js';
	import { Backend } from '$lib/backend.svelte.js';
	import type { Template } from '$lib/types.js';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { data } = $props();

	let isMounted = $state(false);

	let activeNavigation = $derived.by(() => {
		const base = [AppSlide.Hero, AppSlide.Intro, AppSlide.Creation, AppSlide.History];
		return data.activeContext ? [...base, AppSlide.Optimization, AppSlide.Manifest] : base;
	});

	async function selectTemplate(id: number) {
		if (data.activeContext?.template.id !== id) {
			await goto(resolve(`/?template=${id}` as `/`));
		}

		nav.scrollTo(AppSlide.Optimization);
	}

	let builder = $derived.by(() => {
		if (!data.activeContext) return null;

		let initialTemplate: Template = {
			id: data.activeContext.template.id,
			name: data.activeContext.template.name,
			class_id: data.activeContext.template.class_id,
			preferences: {},
			equipped_items: {}
		};

		if (browser) {
			const rawSave = window.localStorage.getItem(data.activeContext.storageKey);
			if (rawSave) {
				try {
					initialTemplate = JSON.parse(rawSave);
				} catch (e) {
					console.error('Save file corrupted, starting fresh.', e);
				}
			}
		}

		return new TemplateBuilder(
			initialTemplate,
			data.activeContext.templateClass,
			data.activeContext.items,
			data.gems,
			data.stats
		);
	});

	let backend = $derived.by(() => {
		if (!builder || !data.activeContext) return null;

		return new Backend((result) => {
			builder.applyOptimizationResult(result.equipped_items, result.equipped_gems);
		});
	});

	$effect(() => {
		if (!browser || !builder || !data.activeContext) return;

		window.localStorage.setItem(data.activeContext.storageKey, JSON.stringify(builder.template));
	});

	onMount(() => {
		nav.init();
		requestAnimationFrame(() => {
			isMounted = true;
		});

		if (!backend) return;

		backend.connect();
		return () => backend.disconnect();
	});
</script>

<div
	class="fixed top-0 z-50 w-full transition-all duration-700 ease-in-out {nav.activeSlide >
	AppSlide.Hero
		? 'translate-y-0 opacity-100'
		: 'pointer-events-none -translate-y-full opacity-0'}"
>
	<Header
		templateName={data.activeContext?.template.name || null}
		showContext={nav.activeSlide !== AppSlide.Hero && data.activeContext !== null}
	/>
</div>

<div
	class="transition-opacity duration-700 {nav.activeSlide > AppSlide.Hero
		? 'opacity-100'
		: 'pointer-events-none opacity-0'}"
>
	<NavigationBar slides={activeNavigation} />
</div>

<div
	class="no-scrollbar h-dvh w-full snap-y snap-mandatory snap-normal overflow-x-hidden overflow-y-auto bg-background pb-19 transition-opacity duration-300 lg:pb-0 {isMounted
		? 'opacity-100'
		: 'opacity-0'}"
>
	<Slide index={AppSlide.Hero}>
		<Hero onBegin={() => nav.scrollTo(AppSlide.Intro)} />
	</Slide>

	<Slide index={AppSlide.Intro}>
		<div class="flex h-full w-full items-center justify-center">
			<h2 class="font-sovereign text-2xl text-foreground-secondary uppercase">Introduction</h2>
		</div>
	</Slide>

	<Slide index={AppSlide.Creation}>
		<TemplateCreation realms={data.realms} classes={data.classes} />
	</Slide>

	<Slide index={AppSlide.History}>
		<TemplateHistory templates={data.templates} classes={data.classes} onSelect={selectTemplate} />
	</Slide>

	{#if data.activeContext && builder && backend}
		<Slide index={AppSlide.Optimization}>
			<Optimization {builder} {backend} />
		</Slide>

		<Slide index={AppSlide.Manifest}>
			<div class="flex h-full w-full items-center justify-center">
				<div class="font-sovereign text-primary">Manifest Placeholder</div>
			</div>
		</Slide>
	{/if}
</div>
