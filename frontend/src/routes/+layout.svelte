<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import Header from '$lib/components/Header.svelte';
    import { Database, LoaderCircle } from 'lucide-svelte';
    let { data, children } = $props();

    let isInitializing = $state(false);

    async function handleInitialize() {
        isInitializing = true;
        await fetch('http://localhost:3000/init', { method: 'POST' });
        window.location.reload(); 
    }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="base-scrollbar flex h-screen w-full flex-col overflow-hidden bg-surface-lowest text-foreground selection:bg-primary/30">
    <Header />

    <main class="flex-1 w-full flex items-center justify-center p-12">
        {#if !data.isInitialized}
            <div class="relative flex w-full max-w-xl flex-col items-center justify-center gap-8 p-10 
                bg-surface-container outline outline-outline rounded-sm">
                
                <Database size={48} class="text-primary relative z-10" strokeWidth={1.5} />

                <div class="space-y-3">
                    <h1 class="font-technical text-3xl font-bold text-center tracking-tight uppercase sm:text-4xl text-foreground">
                        Database Initialization
                    </h1>
                    <p class="text-sm text-foreground-secondary/50 text-center">
                        The database is currently uninitialized.
                    </p>
                    <p>
                        Make sure you have the necessary raw data in the 
                        <code class="font-mono bg-surface-lowest px-1 py-0.5 rounded-sm text-foreground-secondary">data/raw/</code> 
                        directory of the backend before initializing.
                    </p>
                </div>
                
                <button 
                    onclick={handleInitialize} 
                    disabled={isInitializing} 
                    class="
                        group relative inline-flex w-full cursor-pointer items-center justify-center overflow-hidden rounded-sm bg-primary 
                        px-8 py-4 font-technical text-sm font-bold uppercase tracking-widest text-background transition-all hover:brightness-110 
                        disabled:cursor-not-allowed disabled:opacity-50 active:scale-[0.98]
                    "
                >
                    {#if isInitializing}
                        <LoaderCircle size={18} class="mr-3 animate-spin" />
                        <span class="animate-pulse">Parsing Data Structures...</span>
                    {:else}
                        <span>Initialize Database</span>
                    {/if}
                </button>
            </div>
        {:else}
            <div class="w-full h-full">
                {@render children()}
            </div>
        {/if}
    </main>
</div>