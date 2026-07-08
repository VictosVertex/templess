<script>
  import { LucideCircle } from 'lucide-svelte';
import { fade } from 'svelte/transition';
    let step = 0;

  /**
	 * @param {{ code: string; preventDefault: () => void; }} e
	 */
  function handleKeydown(e) {
    if (e.code === 'Space') {
      e.preventDefault();
      
      if (step < 4) {
        step++;
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="flex flex-col justify-center h-full pl-16 md:pl-32 pb-20 text-3xl md:text-6xl">
  <ul class="space-y-9">
    
    <li class="flex items-center gap-6">
      <LucideCircle class="text-[#9DA322] mt-2" size={30}/>
      <span>Motivation</span>
    </li>
    
    {#if step >= 1}
      <li transition:fade={{ duration: 300 }} class="flex items-center gap-6">
        <LucideCircle class="text-[#9DA322] mt-2" size={30}/>
        <span>Problem</span>
      </li>
    {/if}
    
    {#if step >= 2}
      <li transition:fade={{ duration: 300 }} class="flex items-center gap-6">
        <LucideCircle class="text-[#9DA322] mt-2" size={30}/>
        <span>Current State</span>
      </li>
    {/if}

    {#if step >= 3}
      <li transition:fade={{ duration: 300 }} class="flex items-center gap-6">
        <LucideCircle class="text-[#9DA322] mt-2" size={30}/>
        <span>Templess</span>
      </li>
    {/if}

    {#if step >= 4}
      <li transition:fade={{ duration: 300 }} class="flex items-center gap-6">
        <LucideCircle class="text-[#9DA322] mt-2" size={30}/>
        <span>Conclusion</span>
      </li>
    {/if}
    
  </ul>
</div>