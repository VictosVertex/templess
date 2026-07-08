<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';

    import { nav, AppSlide } from '$lib/navigation.svelte';
    import { TemplateBuilder } from '$lib/template-builder.svelte.js';
    import { Backend } from '$lib/backend.svelte.js';
    import { api } from '$lib/api';
    
    import type { Template, EquippedItemState, StatPreference, OptimizationContext, Gem, StatDefinition } from '$lib/types.js';

    import Header from '$lib/components/Header.svelte';
    import NavigationBar from '$lib/components/NavigationBar.svelte';
    import Slide from '$lib/components/Slide.svelte';
    import Title from '$lib/slides/presentation/Title.svelte';
    import OverviewSlide from '$lib/slides/presentation/OverviewSlide.svelte';
    import PresentationSlide from '$lib/slides/presentation/PresentationSlide.svelte';
    import MotivationSlide from '$lib/slides/presentation/MotivationSlide.svelte';
    import Hero from '$lib/slides/Hero.svelte';
    import Checkout from '$lib/slides/Checkout.svelte';
    import TemplateCreation from '$lib/slides/TemplateCreation.svelte';
    import TemplateHistory from '$lib/slides/TemplateHistory.svelte';
    import Optimization from '$lib/slides/optimization/Optimization.svelte';
	import DAoCSlide from '$lib/slides/presentation/DAoCSlide.svelte';
	import SourceSlide from '$lib/slides/presentation/SourceSlide.svelte';
	import StatsSlide from '$lib/slides/presentation/StatsSlide.svelte';
	import CurrentStateSlide from '$lib/slides/presentation/CurrentStateSlide.svelte';

    type TemplateDraft = {
        class_id: number;
        preferences?: Record<number, StatPreference>;
        equipped_items?: Record<number, EquippedItemState>;
    };

    function parseTemplateDraft(rawSave: string): TemplateDraft | null {
        const parsedSave = JSON.parse(rawSave);

        if (typeof parsedSave !== 'object' || parsedSave === null) {
            return null;
        }

        if (typeof parsedSave.class_id !== 'number') {
            return null;
        }

        return {
            class_id: parsedSave.class_id,
            preferences: parsedSave.preferences,
            equipped_items: parsedSave.equipped_items
        };
    }

    let { data } = $props();

    let isMounted = $state(false);
    let persistedTemplate = $state<Template | null>(null);
    let builder = $state.raw<TemplateBuilder | null>(null);
    let backend = $state.raw<Backend | null>(null);

    let activeNavigation = $derived.by(() => {
        const base = [AppSlide.Hero, AppSlide.Intro, AppSlide.Creation, AppSlide.History];
        return data.activeContext ? [...base, AppSlide.Optimization, AppSlide.Checkout] : base;
    });

    $effect(() => {
        const ctx = data.activeContext;
        if (!ctx) {
            builder = null;
            return;
        }

        if (builder?.template.id !== ctx.template.id) {
            persistedTemplate = ctx.template;
            builder = createHydratedBuilder(ctx, data.gems, data.stats);
        }
    });

    $effect(() => {
        if (!builder || !data.activeContext) {
            backend = null;
            return;
        }

        const activeBackend = new Backend((result) => {
            builder?.applyOptimizationResult(result.equipped_items, result.equipped_gems);
        });

        activeBackend.connect();
        backend = activeBackend;

        return () => {
            activeBackend.disconnect();

            if (backend === activeBackend) {
                backend = null;
            }
        };
    });

    $effect(() => {
        if (!browser || !builder || !data.activeContext || builder.draftRevision === 0) return;

        window.localStorage.setItem(
            data.activeContext.storageKey,
            JSON.stringify({
                class_id: builder.template.class_id,
                preferences: builder.template.preferences,
                equipped_items: builder.template.equipped_items
            })
        );
    });

    onMount(() => {
        nav.init();
        requestAnimationFrame(() => {
            isMounted = true;
        });
    });

    async function selectTemplate(id: number) {
        if (data.activeContext?.template.id !== id) {
            await goto(resolve(`/?template=${id}` as `/`));
        }
        nav.scrollTo(AppSlide.Optimization);
    }

    function discardDraft() {
        if (!browser || !builder || !data.activeContext) return;
        window.localStorage.removeItem(data.activeContext.storageKey);
        if (persistedTemplate) builder.resetToTemplate(persistedTemplate);
    }

    async function saveTemplate() {
        if (!builder || !data.activeContext) return;
        persistedTemplate = await api.updateTemplate(builder.template.id, {
            preferences: builder.template.preferences,
            equipped_items: builder.template.equipped_items
        });
        if (browser) window.localStorage.removeItem(data.activeContext.storageKey);
        builder.resetToTemplate(persistedTemplate);
    }

    function createHydratedBuilder(ctx: OptimizationContext, gems: Record<number, Gem>, stats: Record<number, StatDefinition>): TemplateBuilder {
        let initialTemplate: Template = {
            ...ctx.template,
            preferences: { ...ctx.template.preferences },
            equipped_items: { ...ctx.template.equipped_items }
        };
        let hasStoredDraft = false;

        if (browser) {
            const rawSave = window.localStorage.getItem(ctx.storageKey);
            if (rawSave) {
                try {
                    const parsedSave = parseTemplateDraft(rawSave);

                    if (parsedSave?.class_id === ctx.template.class_id) {
                        initialTemplate = {
                            ...initialTemplate,
                            preferences: { ...initialTemplate.preferences, ...(parsedSave.preferences ?? {}) },
                            equipped_items: { ...initialTemplate.equipped_items, ...(parsedSave.equipped_items ?? {}) }
                        };
                        hasStoredDraft = true;
                    } else {
                        window.localStorage.removeItem(ctx.storageKey);
                    }
                } catch {
                    window.localStorage.removeItem(ctx.storageKey);
                }
            }
        }

        const nextBuilder = new TemplateBuilder(initialTemplate, ctx.templateClass, ctx.items, gems, stats);
        if (hasStoredDraft) nextBuilder.draftRevision = 1;
        return nextBuilder;
    }
</script>

<div class="fixed top-0 z-50 w-full transition-all duration-700 ease-in-out {nav.activeSlide > AppSlide.Hero ? 'translate-y-0 opacity-100' : 'pointer-events-none -translate-y-full opacity-0'}">
    <Header
        templateName={data.activeContext?.template.name || null}
        className={data.activeContext?.templateClass.name || null}
        showContext={nav.activeSlide !== AppSlide.Hero && data.activeContext !== null}
    />
</div>

<div class="transition-opacity duration-700 {nav.activeSlide > AppSlide.Hero ? 'opacity-100' : 'pointer-events-none opacity-0'}">
    <NavigationBar slides={activeNavigation} />
</div>

<div class="h-dvh w-full snap-y snap-mandatory snap-normal overflow-x-hidden overflow-y-auto bg-background pb-19 transition-opacity duration-300 motion-safe:scroll-smooth lg:pb-0 {nav.activeSlide < AppSlide.Hero ? 'no-scrollbar' : ''} {isMounted ? 'opacity-100' : 'opacity-0'}">
    
    <Slide index={AppSlide.Title}>
        <Title />
    </Slide>

    <Slide index={AppSlide.OverviewSlide}>
        <PresentationSlide index={AppSlide.OverviewSlide}>
            <OverviewSlide />
        </PresentationSlide>
    </Slide>  

    <Slide index={AppSlide.MotivationSlide}>
        <PresentationSlide index={AppSlide.MotivationSlide}>
            <MotivationSlide isActive={nav.activeSlide === AppSlide.MotivationSlide} />
        </PresentationSlide>
    </Slide>

	<Slide index={AppSlide.DaocSlide}>
        <PresentationSlide index={AppSlide.DaocSlide}>
            <DAoCSlide isActive={nav.activeSlide === AppSlide.DaocSlide} />
        </PresentationSlide>
    </Slide>

	<Slide index={AppSlide.SourceSlide}>
        <PresentationSlide index={AppSlide.SourceSlide}> 
            <SourceSlide isActive={nav.activeSlide === AppSlide.SourceSlide} />
        </PresentationSlide>
    </Slide>

	<Slide index={AppSlide.StatsSlide}>
        <PresentationSlide index={AppSlide.StatsSlide}>
            <StatsSlide isActive={nav.activeSlide === AppSlide.StatsSlide} />
        </PresentationSlide>
    </Slide>

	<Slide index={AppSlide.CurrentStateSlide}>
        <PresentationSlide index={AppSlide.CurrentStateSlide}>
            <CurrentStateSlide isActive={nav.activeSlide === AppSlide.CurrentStateSlide} />
        </PresentationSlide>
    </Slide>

    <Slide index={AppSlide.Hero}>
        <Hero onBegin={() => nav.scrollTo(AppSlide.Intro)} />
    </Slide>

    <Slide index={AppSlide.Intro}>
        <div class="flex h-full w-full items-center justify-center">
            <h2 class="font-sovereign text-2xl text-foreground-secondary uppercase">Introduction</h2>
        </div>
    </Slide>

    <Slide index={AppSlide.Creation}>
        <TemplateCreation
            realms={data.realms}
            classes={data.classes}
            preferencePresets={data.preferencePresets}
        />
    </Slide>

    <Slide index={AppSlide.History}>
        <TemplateHistory templates={data.templates} classes={data.classes} onSelect={selectTemplate} />
    </Slide>

    {#if data.activeContext && builder && backend}
        <Slide index={AppSlide.Optimization}>
            <Optimization
                {builder}
                {backend}
                onDiscardDraft={discardDraft}
                onSaveTemplate={saveTemplate}
            />
        </Slide>

        <Slide index={AppSlide.Checkout}>
            <Checkout {builder} />
        </Slide>
    {/if}
</div>