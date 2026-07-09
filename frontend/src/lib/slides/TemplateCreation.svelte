<script lang="ts">
    import { browser } from '$app/environment';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import { api } from '$lib/api';
    import Button from '$lib/components/Button.svelte';
    import { realmTheme } from '$lib/constants';
    import type { Realm, ClassResponse, PreferencePreset } from '$lib/types';
    import { nav, AppSlide } from '$lib/navigation.svelte';
    import { LoaderCircle } from 'lucide-svelte';
    import OrnamentSeparator from '$lib/components/OrnamentSeparator.svelte';

    let {
        realms,
        classes,
        preferencePresets
    }: {
        realms: Realm[];
        classes: ClassResponse[];
        preferencePresets: Partial<Record<number, PreferencePreset[]>>;
    } = $props();

    let selectedRealmId = $state<number | null>(null);
    let selectedClassId = $state<number | null>(null);
    let selectedPreferencePresetId = $state<number | null>(null);
    let templateName = $state('');
    let isCreating = $state(false);

    let filteredClasses = $derived(
        selectedRealmId ? classes.filter((c) => c.realm_id === Number(selectedRealmId)) : []
    );
    let filteredPreferencePresets = $derived(
        selectedClassId ? preferencePresets[selectedClassId] ?? [] : []
    );

    function handleRealmChange() {
        selectedClassId = null;
        selectedPreferencePresetId = null;
    }

    function handleClassChange() {
        selectedPreferencePresetId = null;
    }

    async function handleCreate(e: Event) {
        e.preventDefault();
        
        if (!selectedClassId || !templateName.trim()) return;

        isCreating = true;

        const presetId = selectedPreferencePresetId ? Number(selectedPreferencePresetId) : null;
        const selectedPreset = presetId 
            ? filteredPreferencePresets.find((preset) => preset.id === presetId) 
            : null;

        const templateId = await api.createTemplate({
            name: templateName.trim(),
            class_id: selectedClassId,
            preference_preset_id: presetId
        });

        if (browser && selectedPreset) {
            window.localStorage.setItem(
                `template-draft:${templateId}`,
                JSON.stringify({
                    class_id: selectedClassId,
                    preferences: selectedPreset.preferences,
                    equipped_items: {}
                })
            );
        }

        await goto(resolve(`/?template=${templateId}` as `/`));
        nav.scrollTo(AppSlide.Optimization);
    }
</script>

<div class="flex w-full max-w-5xl flex-col items-center justify-center gap-12 max-[400px]:gap-3 lg:gap-24">
    <p class="max-w-lg text-center font-reading text-[10px] font-bold leading-relaxed tracking-[0.3em] text-foreground-secondary uppercase sm:text-xs">
        Create a new optimization template by selecting a realm, class and name.
    </p>

    <form onsubmit={handleCreate} class="flex w-full flex-col gap-2 md:gap-10">
        <div class="grid w-full grid-cols-3 gap-3 md:gap-6">
            {#each realms as realm (realm.id)}
                {@const Icon = realmTheme[realm.id].icon}
                {@const color = realmTheme[realm.id].color}
                
                <label class="group relative cursor-pointer">
                    <input
                        type="radio"
                        name="realm"
                        value={realm.id}
                        bind:group={selectedRealmId}
                        onchange={handleRealmChange}
                        class="peer sr-only"
                    />
                    <div class="flex flex-col items-center justify-center gap-3 rounded-xl border border-outline bg-transparent px-6 py-4 transition-all peer-checked:border-primary peer-checked:bg-primary/10 hover:border-primary/50 sm:h-36 sm:gap-4 sm:px-4 sm:py-6 md:h-44">
                        <Icon size={26} class="{color} opacity-60 sm:size-10" strokeWidth={1.5} />

                        <span class="font-display text-base tracking-widest uppercase {selectedRealmId === realm.id ? 'text-primary' : 'text-foreground-secondary group-hover:text-foreground'}">
                            <span class="hidden max-[400px]:block">{realm.name.slice(0, 3)}</span>
                            <span class="hidden max-[400px]:hidden sm:block">{realm.name}</span>
                        </span>

                        {#if selectedRealmId === realm.id}
                            <div class="absolute top-3 right-3 h-2 w-2 rotate-45 bg-primary"></div>
                        {/if}
                    </div>
                </label>
            {/each}
        </div>

        <div class="w-full max-[400px]:hidden">
            <OrnamentSeparator />
        </div>

        <div class="grid w-full grid-cols-1 items-end gap-6 lg:grid-cols-[1fr_1fr_1fr_auto] lg:gap-8">
            
            <div class="flex flex-col gap-2">
                <label for="class-select" class="font-reading text-[10px] font-bold tracking-[0.2em] text-primary/70 uppercase">
                    Class
                </label>
                <select
                    id="class-select"
                    bind:value={selectedClassId}
                    onchange={handleClassChange}
                    disabled={!selectedRealmId}
                    class="w-full appearance-none rounded-sm border border-outline bg-transparent px-4 py-3 text-primary transition-colors focus:border-primary focus:outline-none focus:ring-0 focus:ring-primary disabled:cursor-not-allowed disabled:opacity-50"
                >
                    <option value={null}>{selectedRealmId ? 'Choose a class...' : 'Select a realm first...'}</option>
                    {#each filteredClasses as cls (cls.id)}
                        <option value={cls.id}>{cls.name}</option>
                    {/each}
                </select>
            </div>

            <div class="flex flex-col gap-2">
                <label for="preset-select" class="font-reading text-[10px] font-bold tracking-[0.2em] text-primary/70 uppercase">
                    Preference Preset
                </label>
                <select
                    id="preset-select"
                    bind:value={selectedPreferencePresetId}
                    disabled={!selectedClassId}
                    class="w-full appearance-none rounded-sm border border-outline bg-transparent px-4 py-3 text-primary transition-colors focus:border-primary focus:outline-none focus:ring-0 focus:ring-primary disabled:cursor-not-allowed disabled:opacity-50"
                >
                    <option value={null}>{selectedClassId ? 'No preset selected' : 'Select a class first...'}</option>
                    {#each filteredPreferencePresets as preset (preset.id)}
                        <option value={preset.id}>{preset.name}</option>
                    {/each}
                </select>
            </div>

            <div class="flex flex-col gap-2">
                <label for="template-name" class="font-reading text-[10px] font-bold tracking-[0.2em] text-primary/70 uppercase">
                    Template Name
                </label>
                <input
                    type="text"
                    id="template-name"
                    bind:value={templateName}
                    maxlength="60"
                    placeholder="e.g. Solo Bard"
                    class="w-full rounded-sm border-outline bg-transparent px-4 py-3 text-primary transition-colors placeholder:text-primary/20 focus:border-primary focus:outline-none focus:ring-0 focus:ring-primary max-[400px]:text-base"
                />
            </div>

            <div class="w-full pt-2 md:w-auto md:pt-0">
                <Button disabled={!selectedClassId || !templateName.trim() || isCreating} class="w-full px-10 py-3.5">
                    {#if isCreating}
                        <LoaderCircle size={16} class="mr-3 animate-spin" />
                        <span class="animate-pulse tracking-[0.2em]">Creating...</span>
                    {:else}
                        <span class="tracking-[0.2em]">NEW TEMPLATE</span>
                    {/if}
                </Button>
            </div>
        </div>
    </form>

    <div class="mt-2 flex w-full items-center justify-center">
        <button
            type="button"
            onclick={() => nav.scrollTo(AppSlide.History)}
            class="group relative flex items-center justify-center gap-4 bg-transparent px-8 py-2 transition-all"
        >
            <div class="h-px w-6 bg-primary/40 transition-all duration-300 group-hover:w-10 group-hover:bg-primary"></div>
            <span class="font-reading text-[10px] font-bold tracking-[0.2em] text-foreground-secondary uppercase transition-colors group-hover:text-primary">
                Continue with Existing Templates
            </span>
            <div class="h-px w-6 bg-primary/40 transition-all duration-300 group-hover:w-10 group-hover:bg-primary"></div>
        </button>
    </div>
</div>