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
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="grid h-full w-full grid-cols-2 gap-24 py-16 pt-40">
  
  <div class="flex flex-col justify-start space-y-12 border-r border-gray-200 pr-12 transition-opacity duration-300" class:opacity-0={step < 1}>
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Dark Age of Camelot</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Massively Multiplayer Online Role-Playing Game</p>
    </div>
    
    <ul class="space-y-8 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">September 2001</span>
        <span class="text-2xl text-[#8F92AC]">One of the first MMORPGs.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Realm vs. Realm (RvR)</span>
        <span class="text-2xl text-[#8F92AC]">Three-faction, macro-scale open-world PvP.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Asymmetric Design</span>
        <span class="text-2xl text-[#8F92AC]">3 distinct factions, 44 unique classes.</span>
      </li>
    </ul>
  </div>

  <div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 2}>
    <div class="border-l-4 border-[#030c3e] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Gameplay Phases</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">Progression and Endgame</p>
    </div>

    <ul class="space-y-8 pl-12">
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Leveling Phase</span>
        <span class="text-2xl text-[#8F92AC]">Increase base attributes, stat caps and skills.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Gearing Phase</span>
        <span class="text-2xl text-[#8F92AC]">Theory, loot hunting, crafting.</span>
      </li>
      <li class="flex flex-col space-y-2">
        <span class="text-3xl font-medium text-[#030c3e]">Endgame (RvR)</span>
        <span class="text-2xl text-[#8F92AC]">Strong passive and active abilities through a Realm Rank system.</span>
      </li>
    </ul>
  </div>

</div>