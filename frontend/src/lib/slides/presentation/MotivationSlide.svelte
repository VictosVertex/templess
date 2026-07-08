<script lang="ts">
	import { Check, X } from "lucide-svelte";

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

<svelte:window onkeydown={handleKeydown} />

<div class="grid h-full w-full grid-cols-2 gap-24 py-16 pt-40">
  
  <div class="flex flex-col justify-start space-y-12 border-r border-gray-200 pr-12 transition-opacity duration-300" class:opacity-0={step < 1}>
    
    <div class="border-l-4 border-red-500 pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">Ignoring the User</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">The Anti-Pattern</p>
    </div>
    
    <ul class="space-y-10 pl-6">
      <li class="flex items-center space-x-6">
        <X class="shrink-0 text-red-500" size={36} strokeWidth={3} />
        <span class="text-3xl font-medium text-[#030c3e]">"Works on my machine!"</span>
      </li>
      <li class="flex items-center space-x-6">
        <X class="shrink-0 text-red-500" size={36} strokeWidth={3} />
        <span class="text-3xl font-medium text-[#030c3e]">User as the API / Middleman</span>
      </li>
      <li class="flex items-center space-x-6">
        <X class="shrink-0 text-red-500" size={36} strokeWidth={3} />
        <span class="text-3xl font-medium text-[#030c3e]">Only design the happy path</span>
      </li>
    </ul>

  </div>

  <div class="flex flex-col justify-start space-y-12 pl-4 transition-opacity duration-300" class:opacity-0={step < 2}>
    
    <div class="border-l-4 border-[#9DA322] pl-8">
      <h3 class="text-4xl font-medium tracking-wide text-[#030c3e] uppercase">User-Centric</h3>
      <p class="mt-4 text-2xl font-bold text-[#8F92AC]">The Solution</p>
    </div>

    <ul class="space-y-10 pl-6">
      <li class="flex items-center space-x-6">
        <Check class="shrink-0 text-[#9DA322]" size={36} strokeWidth={4} />
        <span class="text-3xl font-medium text-[#030c3e]">Build for reality</span>
      </li>
      <li class="flex items-center space-x-6">
        <Check class="shrink-0 text-[#9DA322]" size={36} strokeWidth={4} />
        <span class="text-3xl font-medium text-[#030c3e]">Clear workflow with sensible defaults</span>
      </li>
      <li class="flex items-center space-x-6">
        <Check class="shrink-0 text-[#9DA322]" size={36} strokeWidth={4} />
        <span class="text-3xl font-medium text-[#030c3e]">Built for failure</span>
      </li>
    </ul>

  </div>

</div>