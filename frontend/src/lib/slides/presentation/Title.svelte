<script>
  import potasscoTechnologiesLogo from '$lib/assets/potassco_technologies.svg';

  export let title = "TempLess (UI): A User-Centric Interface Approach";
  export let authors = "Workshop on UI/UX for ASP 2026";
  export let affiliation = "Dominik Gerndt - Potassco Solutions GmbH";
  
  // Footer props
  export let footerAuthor = "Dominik Gerndt";
  export let footerTitle = "";
  export let slideNumber = "1 / 15";
</script>

<div class="flex h-dvh w-screen items-center justify-center overflow-hidden bg-[#050505]">
  <div class="@container relative aspect-video max-h-dvh w-full max-w-[calc(100dvh*16/9)] flex flex-col overflow-hidden bg-white font-sans text-[#030c3e]">
    
    <main class="flex flex-1 flex-col items-center justify-center px-24 text-center">
      
      <h1 class="max-w-[20ch] text-6xl font-semibold tracking-tight text-[#030c3e] leading-[1.15] md:text-[5.5rem]">
        {title}
      </h1>

      <div class="mt-16 flex flex-col items-center space-y-4">
        <h2 class="text-3xl font-medium text-[#030c3e] md:text-[2.75rem]">
          {authors}
        </h2>
        <h3 class="text-2xl font-normal text-[#8F92AC] md:text-3xl">
          {affiliation}
        </h3>
      </div>

      <div class="mt-24">
        <img
          src={potasscoTechnologiesLogo}
          alt="Potassco Technologies logo"
          class="h-32 w-auto object-contain md:h-40"
        />
      </div>
      
    </main>

    <footer class="flex w-full items-center justify-between border-t border-gray-200 bg-gray-50 px-8 py-4 text-xl text-[#8F92AC] md:text-2xl">
      <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-left font-medium text-[#030c3e]">
        {footerAuthor}
      </div>
      
      <div class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-center">
        {footerTitle}
      </div>
      
      <div class="flex-1 whitespace-nowrap text-right font-medium text-[#030c3e]">
        {slideNumber}
      </div>
    </footer>
    
  </div>
</div>