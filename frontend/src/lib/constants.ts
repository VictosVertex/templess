import { Shield, Axe, Leaf } from 'lucide-svelte';

export const realmTheme: Record<number, { icon: typeof Shield; color: string; bg: string }> = {
	1: { icon: Shield, color: 'text-red-400', bg: 'bg-red-400/10' },
	2: { icon: Axe, color: 'text-blue-400', bg: 'bg-blue-400/10' },
	3: { icon: Leaf, color: 'text-green-400', bg: 'bg-green-400/10' }
};

export const defaultRealmTheme = { icon: Shield, color: 'text-primary', bg: 'bg-primary/10' };
