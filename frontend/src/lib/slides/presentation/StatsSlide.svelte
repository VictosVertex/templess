<script lang="ts">
  // Accept an isActive prop so the slide knows when it's on screen
  let { isActive } = $props<{ isActive: boolean }>();
  
  let step = $state(0);

  function handleKeydown(e: KeyboardEvent) {
    // If the slide is not active, ignore the keystroke entirely
    if (!isActive) return;

    if (e.code === 'Space') {
      e.preventDefault();
      
      if (step < 2) {
        step++;
      }
    }
  }

  const BASE_CAP = 75;
  const TARGET_STRENGTH = 85;

  type DemoItem = {
    id: string;
    name: string;
    description: string;
    strength: number;
    strengthCap: number;
  };

  const items: DemoItem[] = [
    { id: 'i1', name: 'Dropped Ring', description: '+45 Raw Strength', strength: 45, strengthCap: 0 },
    { id: 'i2', name: 'Crafted Chest', description: '+45 Raw Strength', strength: 45, strengthCap: 0 },
    { id: 'i3', name: 'Champion Bracer', description: '+10 Overcap Limit', strength: 0, strengthCap: 10 }
  ];

  // All items start unselected for a clean slate
  let equipped = $state<Record<string, boolean>>({
    'i1': false,
    'i2': false,
    'i3': false
  });

  let equippedItems = $derived(items.filter((item) => equipped[item.id]));
  let totalStrength = $derived(equippedItems.reduce((sum, item) => sum + item.strength, 0));
  let totalStrengthCap = $derived(BASE_CAP + equippedItems.reduce((sum, item) => sum + item.strengthCap, 0));
  let effectiveStrength = $derived(Math.min(totalStrength, totalStrengthCap));
  let wastedStrength = $derived(Math.max(0, totalStrength - totalStrengthCap));

  function totalPercent(value: number, max: number) {
    return Math.min((value / max) * 100, 100);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="grid h-full w-full grid-cols-[1.15fr_1fr] gap-24 py-16">
  
  <div class="flex flex-col justify-start border-r border-gray-200 pr-12">
    
    <div class="flex flex-col">
      {#each items as item (item.id)}
        <button
          type="button"
          class="flex w-full cursor-pointer items-center justify-between border-b border-gray-200 py-6 text-left transition-colors"
          onclick={() => equipped[item.id] = !equipped[item.id]}
        >
          <div class="flex items-center space-x-6">
            <div class="flex h-8 w-8 shrink-0 items-center justify-center border-2 transition-colors {equipped[item.id] ? 'border-[#030c3e] bg-[#030c3e]' : 'border-gray-300'}">
              {#if equipped[item.id]}
                <svg class="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                </svg>
              {/if}
            </div>
            <span class="text-3xl font-medium transition-colors {equipped[item.id] ? 'text-[#030c3e]' : 'text-[#8F92AC]'}">
              {item.name}
            </span>
          </div>
          <span class="text-2xl font-bold uppercase tracking-widest text-[#8F92AC]">
            {item.description}
          </span>
        </button>
      {/each}
    </div>

    <div class="mt-14 flex flex-col space-y-12">
      
      <div class="grid grid-cols-3 divide-x divide-gray-200 border-b border-gray-200 pb-10 text-center">
        <div class="flex flex-col">
          <span class="text-lg font-bold uppercase tracking-widest text-[#8F92AC]">Total Input</span>
          <span class="mt-2 text-6xl font-medium transition-colors {wastedStrength > 0 ? 'text-red-500' : 'text-[#030c3e]'}">
            {totalStrength}
          </span>
        </div>
        <div class="flex flex-col">
          <span class="text-lg font-bold uppercase tracking-widest text-[#8F92AC]">Current Cap</span>
          <span class="mt-2 text-6xl font-medium text-[#030c3e]">
            {totalStrengthCap}
          </span>
        </div>
        <div class="flex flex-col">
          <span class="text-lg font-bold uppercase tracking-widest text-[#8F92AC]">Effective</span>
          <span class="mt-2 text-6xl font-medium text-emerald-500">
            {effectiveStrength}
          </span>
        </div>
      </div>

      <div class="space-y-10">
        <div class="flex flex-col space-y-3">
          <div class="flex justify-between text-2xl font-bold uppercase tracking-widest text-[#030c3e]">
            <span>Net Effective Stat</span>
            <span class="text-emerald-600">{effectiveStrength} / {TARGET_STRENGTH}</span>
          </div>
          <div class="h-3 w-full bg-gray-100">
            <div class="h-full bg-emerald-500 transition-all duration-500 ease-out" style="width: {totalPercent(effectiveStrength, TARGET_STRENGTH)}%"></div>
          </div>
        </div>

        <div class="flex flex-col space-y-3">
          <div class="flex justify-between text-2xl font-bold uppercase tracking-widest text-[#030c3e]">
            <span class="{wastedStrength > 0 ? 'text-red-500' : 'text-[#8F92AC]'}">Wasted Variance</span>
            <span class="text-red-500">{wastedStrength}</span>
          </div>
          <div class="h-3 w-full bg-gray-100">
            <div class="h-full bg-red-500 transition-all duration-500 ease-out" style="width: {totalPercent(wastedStrength, TARGET_STRENGTH)}%"></div>
          </div>
        </div>
      </div>
      
    </div>
  </div>

  <div class="flex flex-col justify-start space-y-12 pl-4">
    
    <div class="flex flex-col space-y-10 transition-opacity duration-500" class:opacity-0={step < 1}>
      <div class="border-l-4 border-[#030c3e] pl-8">
        <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Many Attributes</h3>
        <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Base, Resists, Skills, Bonuses,...</p>
      </div>
      
      <div class="border-l-4 border-gray-200 pl-8">
        <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Caps</h3>
        <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Stats do not scale infinitely.</p>
      </div>

      <div class="border-l-4 border-gray-200 pl-8">
        <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Overcapping</h3>
        <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Some stats can be pushed beyond their limits.</p>
      </div>
    </div>

    <div class="transition-opacity duration-500 pt-8" class:opacity-0={step < 2}>
      <div class="bg-[#030c3e] p-10 text-white shadow-xl">
        <h3 class="text-4xl font-medium tracking-widest uppercase text-white">The Target</h3>
        <p class="mt-4 text-3xl font-bold leading-relaxed text-gray-300">
          A <span class="text-emerald-400">Perfect Template</span>.<br>
          Equipping a collection of items such that all desired attributes are as close to their limit as possible.
        </p>
      </div>
    </div>

  </div>

</div>