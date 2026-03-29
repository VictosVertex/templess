<script lang="ts">
    import type { Realm, ClassResponse } from '$lib/types';
    import { Leaf, Shield, Axe } from 'lucide-svelte';

    let { realms, classes }: { realms: Realm[], classes: ClassResponse[] } = $props();

    let selectedRealmId = $state<number | null>(null);
    let selectedClassId = $state<number | null>(null);
    let templateName = $state('');

    let filteredClasses = $derived(
        selectedRealmId 
            ? classes.filter(c => c.realm_id === Number(selectedRealmId))
            : []
    );

    // // Reset class when realm changes
    // $effect(() => {
    //     if (selectedRealmId) {
    //         selectedClassId = null;
    //     }
    // });

    function handleCreate(e: Event) {
        e.preventDefault();
        console.log("Submitting:", { selectedRealmId, selectedClassId, templateName });
        // TODO: Call POST /templates endpoint here
    }

    const realmData: Record<string, { 
        icon: typeof Shield, 
        color: string }> = {
        1: { icon: Shield, color: 'text-red-400' },
        2: { icon: Axe, color: 'text-blue-400' },
        3: { icon: Leaf, color: 'text-green-400' }
    };
</script>

<div class="flex flex-col items-center gap-6 w-full max-w-2xl mx-auto mt-12">
    <div class="text-center space-y-2">
        <h1 class="font-technical text-3xl font-bold uppercase tracking-tight text-foreground">
            Create A New Template
        </h1>
        <p class="text-sm text-foreground-secondary">
            Select a realm, class, and name for your new template and start optimizing!
        </p>
    </div>

    <div class="w-full bg-surface-container outline outline-outline rounded-sm p-8 sm:p-10">
        <form onsubmit={handleCreate} class="flex flex-col gap-8">
            
            <div class="flex flex-col gap-3">
                <p class="font-technical text-sm font-bold uppercase tracking-widest text-foreground-secondary">
                    1. Select Realm
                </p>
                
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    {#each realms as realm (realm.id)}
                        {@const Icon = realmData[realm.id].icon}
                        {@const color = realmData[realm.id].color}
                        <label class="cursor-pointer relative group">
                            <input 
                                type="radio" 
                                name="realm" 
                                value={realm.id} 
                                bind:group={selectedRealmId} 
                                class="peer sr-only" 
                            />
                            <div class="flex flex-col items-center gap-3 rounded-sm border border-outline bg-surface-lowest p-5 transition-all hover:border-primary/50 peer-checked:border-primary peer-checked:bg-primary/10">
                                <Icon size={32} class={color} strokeWidth={1.5} />
                                
                                <span class="font-technical text-sm font-bold tracking-wider {selectedRealmId === realm.id ? 'text-primary' : 'text-foreground'}">
                                    {realm.name}
                                </span>
                                
                                {#if selectedRealmId === realm.id}
                                    <div class="absolute top-3 right-3 h-2 w-2 rounded-full bg-primary animate-pulse"></div>
                                {/if}
                            </div>
                        </label>
                    {/each}
                </div>
            </div>

            <div class="flex flex-col gap-3">
                <label for="class-select" class="font-technical text-sm font-bold uppercase tracking-widest text-foreground-secondary">
                    2. Select Class
                </label>
                <div class="relative">
                    <select 
                        id="class-select" 
                        bind:value={selectedClassId}
                        disabled={!selectedRealmId}
                        class="w-full appearance-none rounded-sm border border-outline bg-surface-lowest px-4 py-3 text-foreground transition-colors focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary disabled:cursor-not-allowed disabled:opacity-50"
                    >
                        <option value={null}>
                            {selectedRealmId ? 'Choose a class...' : 'Select a realm first...'}
                        </option>
                        {#each filteredClasses as cls (cls.id)}
                            <option value={cls.id}>{cls.name}</option>
                        {/each}
                    </select>
                    <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-foreground-secondary">
                        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                    </div>
                </div>
            </div>

            <div class="flex flex-col gap-3">
                <label for="template-name" class="font-technical text-sm font-bold uppercase tracking-widest text-foreground-secondary">
                    3. Template Name
                </label>
                <input 
                    type="text" 
                    id="template-name" 
                    bind:value={templateName}
                    class="w-full rounded-sm border border-outline bg-surface-lowest px-4 py-3 text-foreground transition-colors placeholder:text-foreground-secondary/50 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary" 
                    placeholder="e.g. Solo Bard" 
                />
            </div>

            <button 
                type="submit" 
                disabled={!selectedClassId || !templateName.trim()}
                class="group relative mt-2 inline-flex w-full cursor-pointer items-center justify-center overflow-hidden rounded-sm bg-primary px-8 py-4 font-technical text-sm font-bold uppercase tracking-widest text-background transition-all hover:brightness-110 active:scale-[0.98] disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:brightness-100"
            >
                New Template
            </button>
            
        </form>
    </div>
</div>