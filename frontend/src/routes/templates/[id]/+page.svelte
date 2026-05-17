<script lang="ts">
    import { browser } from '$app/environment';
    import Modal from '$lib/components/Modal.svelte';
    import { SLOT_NAMES, defaultRealmTheme, realmTheme } from '$lib/constants';
    import {
        EquipSource,
        TemplateBuilder,
        type TemplateBuilderSnapshot
    } from '$lib/template-builder.svelte';
    import { ItemSlot, type Item, type OptimizationResult, type StatPreference } from '$lib/types';
    import { onMount } from 'svelte';
    import Attributes from './components/Attributes.svelte';
    import Inventory from './components/Inventory.svelte';
    import ItemSelector from './components/ItemSelector.svelte';
    import Preferences from './components/Preferences.svelte';
    import { Backend } from '$lib/backend.svelte.js';
    import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
    import Button from '$lib/components/Button.svelte';
    import { History, Save, SlidersHorizontal } from 'lucide-svelte';

    let { data } = $props();

    let activeSlot = $state<ItemSlot | null>(null);
    let isEditingPreferences = $state<boolean>(false);

    const templateClass = $derived.by(() => {
        const foundClass = data.classes.find((c) => c.id === data.template.class_id);

        if (!foundClass) {
            throw new Error('Class not found for template');
        }

        return foundClass;
    });

    const builder = new TemplateBuilder(
        () => data.stats,
        () => templateClass
    );

    let theme = $derived(realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme);
    let Icon = $derived(theme.icon);

    const itemDictionary = $derived(Object.fromEntries(data.items.map((item) => [item.id, item])));
    const gemDictionary = $derived(Object.fromEntries(data.gems.map((gem) => [gem.id, gem])));
    const storageKey = $derived(`template-draft:${data.template.id}`);

    let modelTimer = $state<number>(0);
    let modelTimerInterval: ReturnType<typeof setInterval> | null = null;

    let fullTimer = $state<number>(0);
    let fullTimerInterval: ReturnType<typeof setInterval> | null = null;

    function startModelTimer() {
        modelTimer = 0;
        if (modelTimerInterval) clearInterval(modelTimerInterval);
        let last = performance.now();
        modelTimerInterval = setInterval(() => {
            const now = performance.now();
            modelTimer += now - last;
            last = now;
        }, 16);
    }

    function resetModelTimer() {
        modelTimer = 0;
    }

    function stopModelTimer() {
        if (modelTimerInterval) {
            clearInterval(modelTimerInterval);
            modelTimerInterval = null;
        }
    }

    function startFullTimer() {
        fullTimer = 0;
        if (fullTimerInterval) clearInterval(fullTimerInterval);
        let last = performance.now();
        fullTimerInterval = setInterval(() => {
            const now = performance.now();
            fullTimer += now - last;
            last = now;
        }, 16);
    }

    function stopFullTimer() {
        if (fullTimerInterval) {
            clearInterval(fullTimerInterval);
            fullTimerInterval = null;
        }
    }

    const backend = new Backend((result: OptimizationResult) => {
        resetModelTimer();
        startModelTimer();
        const inflatedItems: Partial<Record<ItemSlot, Item>> = {};
        const inflatedGems: Partial<Record<ItemSlot, typeof data.gems>> = {};

        for (const [slotStr, itemId] of Object.entries(result.equipped_items)) {
            const slot = parseInt(slotStr, 10) as ItemSlot;
            const fullItem = itemDictionary[itemId];

            if (fullItem) {
                inflatedItems[slot] = fullItem;
            }

            inflatedGems[slot] = (result.equipped_gems[itemId] ?? [])
                .map((gemId) => gemDictionary[gemId])
                .filter((gem) => gem !== undefined);
        }

        builder.applyOptimizationResult(inflatedItems, inflatedGems);
    });

    onMount(() => {
        if (browser) {
            const rawSnapshot = window.localStorage.getItem(storageKey);

            if (rawSnapshot) {
                try {
                    builder.restoreSnapshot(
                        JSON.parse(rawSnapshot) as TemplateBuilderSnapshot,
                        itemDictionary
                    );
                } catch (error) {
                    console.warn('Failed to restore template draft from localStorage', error);
                }
            }
        }

        backend.connect();
        return () => backend.disconnect();
    });

    $effect(() => {
        if (!browser) {
            return;
        }

        window.localStorage.setItem(storageKey, JSON.stringify(builder.toSnapshot()));
    });

    function handleOpenSlot(slot: ItemSlot) {
        activeSlot = slot;
    }

    function handleCloseItemSelection() {
        activeSlot = null;
    }

    function handleItemSelection(item: Item) {
        if (activeSlot !== null) {
            builder.equipItem(activeSlot, item, EquipSource.User);
            handleCloseItemSelection();
        }
    }

    function handleClosePreferences() {
        isEditingPreferences = false;
    }

    function handleOpenPreferences() {
        isEditingPreferences = true;
    }

    function handleSavePreferences(preferences: Record<number, StatPreference>) {
        builder.setPreferences(preferences);
        handleClosePreferences();
    }
</script>

<div class="mx-auto flex w-full max-w-[1600px] flex-col items-center gap-10 px-4 py-8 sm:px-8">
    
    <div class="w-full">
        <OrnamentHeader>
            <div class="flex flex-row items-center justify-center gap-4">
                <span class="font-bold tracking-[0.2em] text-primary uppercase">{data.template.name}</span>
                <Icon size={20} class={theme.color} strokeWidth={1.5} />
                <span class="text-xs tracking-[0.25em] text-foreground-secondary uppercase">{templateClass.name}</span>
            </div>
        </OrnamentHeader>
    </div>

    <div class="grid w-full grid-cols-1 items-start gap-12 lg:grid-cols-[320px_minmax(0,1fr)_320px] lg:gap-8 xl:gap-16">
        
        <div class="flex w-full flex-col gap-12">
            <Attributes title="Base Stats" stats={builder.buckets.baseStats} />
            <Attributes title="Resists" stats={builder.buckets.resists} />
        </div>

        <div class="relative flex w-full flex-col items-center justify-start pt-2">
            
            <div class="absolute flex items-center gap-6 opacity-90 mb-8">
                <div class="flex items-center gap-3 rounded-sm border border-outline/30 bg-surface-low/50 px-4 py-1.5 backdrop-blur-sm">
                    <span class="font-technical text-[10px] font-bold tracking-widest text-foreground-secondary uppercase">Full</span>
                    <span class="font-mono text-xs tracking-wider text-foreground">
                        {(() => {
                            const totalMs = Math.floor(fullTimer);
                            const minutes = Math.floor(totalMs / 60000);
                            const seconds = Math.floor((totalMs % 60000) / 1000);
                            const ms = Math.floor((totalMs % 1000) / 10);
                            return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
                        })()}
                    </span>
                </div>
                <div class="flex items-center gap-3 rounded-sm border border-primary/20 bg-primary/5 px-4 py-1.5 backdrop-blur-sm">
                    <span class="font-technical text-[10px] font-bold tracking-widest text-primary/70 uppercase">Model</span>
                    <span class="font-mono text-xs tracking-wider text-primary">
                        {(() => {
                            const totalMs = Math.floor(modelTimer);
                            const minutes = Math.floor(totalMs / 60000);
                            const seconds = Math.floor((totalMs % 60000) / 1000);
                            const ms = Math.floor((totalMs % 1000) / 10);
                            return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
                        })()}
                    </span>
                </div>
            </div>

            <Inventory
                onOpenSlot={handleOpenSlot}
                {builder}
                {backend}
                stats={data.stats}
                startTimer={startModelTimer}
                stopTimer={stopModelTimer}
                {startFullTimer}
                {stopFullTimer}
            />

            <div class="mt-8 flex items-center justify-center gap-6">
                <Button variant="pill" onClick={() => {}}>
                    <Save size={14} /> Save
                </Button>
                <Button variant="pill" onClick={handleOpenPreferences}>
                    <SlidersHorizontal size={14} /> Preferences
                </Button>
                <Button variant="pill" onClick={() => {}}>
                    <History size={14} /> History
                </Button>
            </div>
        </div>

        <div class="flex w-full flex-col gap-12">
            <Attributes title="Skills" stats={builder.buckets.skills} />
            <Attributes
                title="Bonuses"
                stats={builder.buckets.bonuses.filter((stat) => stat.value !== 0)}
            />
        </div>
    </div>

    </div>
<Modal
    isOpen={activeSlot !== null}
    onClose={handleCloseItemSelection}
    title="Select an Item for {SLOT_NAMES[activeSlot || ItemSlot.Chest]}"
>
    <ItemSelector
        items={data.items}
        targetSlot={activeSlot}
        currentItem={activeSlot !== null ? (builder.equippedItems[activeSlot]?.item ?? null) : null}
        stats={data.stats}
        onSelect={handleItemSelection}
    />
</Modal>

<Modal
    isOpen={isEditingPreferences}
    onClose={handleClosePreferences}
    title="Customize stat preferences"
>
    <Preferences
        stats={Object.values(data.stats)}
        template_class={builder.getTemplateClass()}
        onSave={handleSavePreferences}
        initialPreferences={builder.preferences}
    />
</Modal>