<script lang="ts">
    import type { Stat } from '$lib/types';
    import Attribute from './Attribute.svelte';

    let { title, stats }: { title: string; stats: Stat[] } = $props();
</script>

<div class="flex w-full flex-col gap-5">
    <div class="flex items-center gap-3 opacity-80">
        <div class="h-px flex-1 bg-linear-to-r from-transparent to-primary/40"></div>
        <h3 class="font-technical text-[11px] font-bold tracking-[0.25em] text-primary uppercase">
            {title}
        </h3>
        <div class="h-px flex-1 bg-linear-to-l from-transparent to-primary/40"></div>
    </div>

    <div class="flex flex-col gap-4 px-8">
        {#each stats as stat (stat.name)}
            <Attribute {stat} />
        {/each}
    </div>
</div>