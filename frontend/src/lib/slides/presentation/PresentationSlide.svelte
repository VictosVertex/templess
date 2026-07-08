<script lang="ts">
  import { SlideLabels, type AppSlide } from "$lib/navigation.svelte";
  import type { Snippet } from "svelte";

  let {
    index,
    children
  }: {
    index: AppSlide;
    children: Snippet;
  } = $props();

  let title = $derived(SlideLabels[index]);
  let slideNumber = $derived(index + 1);

  let footerAuthor = "Dominik Gerndt";
  let footerTitle = $derived("TempLess (UI) \u2013 " + title);

  let slideMaxNumber = 1337;
</script>

<div class="flex h-dvh w-screen items-center justify-center overflow-hidden bg-[#050505]">
  <div class="@container relative flex aspect-video max-h-dvh w-full max-w-[calc(100dvh*16/9)] flex-col overflow-hidden bg-white font-sans text-[#030c3e]">
    
    <header class="px-16 pt-12">
      {#if title}
        <h2 class="text-5xl font-semibold tracking-tight text-[#030c3e] md:text-[4rem]">
          {title}
        </h2>
      {/if}
    </header>

    <main class="flex-1 w-full px-16">
      {@render children()} 
    </main>

    <footer class="flex w-full items-center justify-between border-t border-gray-200 bg-gray-50 px-8 py-4 text-xl text-[#8F92AC] md:text-2xl">
      <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-left font-medium text-[#030c3e]">
        {footerAuthor}
      </div>
      
      <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-center">
        {footerTitle}
      </div>
      
      <div class="flex-1 whitespace-nowrap text-right font-medium text-[#030c3e]">
        {slideNumber} / {slideMaxNumber}
      </div>
    </footer>
    
  </div>
</div>