<script lang="ts">
    import type { Stat } from '$lib/types';

    let { stat }: { stat: Stat } = $props();

    let fillPercentage = $derived(Math.min((stat.value / stat.currentCap) * 100, 100));
    let minPercentage = $derived(Math.min((stat.min / stat.currentCap) * 100, 100));

    let colorClass = $derived.by(() => {
        if (stat.value >= stat.currentCap) return 'text-success';
        if (stat.min > 0 && stat.value >= stat.min) return 'text-warning';
        return 'text-foreground-secondary';
    });

    let barColorClass = $derived.by(() => {
        if (stat.min > 0 && stat.value < stat.min) return 'bg-error';
        if (stat.min > 0 && stat.value >= stat.min) return 'bg-success';
        return 'bg-primary';
    });
</script>

<div class="flex flex-col gap-1.5">
    <div class="flex items-baseline justify-between">
        <span class="font-technical text-sm tracking-wider text-foreground capitalize">
            {stat.name.replace(/_/g, ' ')}
        </span>
        
        <span class="font-technical text-xs font-bold tracking-widest {colorClass}">
            {stat.value}<span class="text-foreground-secondary/40">/{stat.currentCap}</span>
        </span>
    </div>
    
    <div class="relative h-0.5 w-full rounded-full bg-primary/10">
        
        <div
            class="h-full rounded-full transition-all duration-300 {barColorClass}"
            style="width: {fillPercentage}%;"
        ></div>
        
        {#if stat.min > 0}
            <div
                class="absolute top-1/2 h-2.5 w-0.5 -translate-x-1/2 -translate-y-1/2
                rounded-full transition-all duration-300
                {stat.value >= stat.min ? 'bg-success' : 'bg-primary/40'}"
                style="left: {minPercentage}%;"
            ></div>
        {/if}
    </div>
</div>