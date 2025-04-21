<script>
    import { base } from '$app/paths';
    import { goto } from '$app/navigation';
    import { AppUserRole } from '$lib/common';
	import { auth } from '$lib/userState.svelte';

	let { children } = $props();
</script>

<style>
	ul#menu li{
		display : inline;
	}
</style>


<ul id="menu">
	<li><a href="{base}/">Domov</a></li> |
	<li><a href="{base}/about">O aplikácií</a></li> |
	{#if auth.userState.role == AppUserRole.Admin}
	<li style="color:indianred">Admin</li> |
	<li><a style="color:indianred" href="{base}/admin/user">Správa používateľov</a></li> |
	<li><a style="color:indianred" href="{base}/admin/enum">Správa číselníkov</a></li> |
	|
	{/if}
	{#if auth.userState.role == AppUserRole.User}
	<li style="color:#117a65">Užívateľ</li> |
	<li><a style="color:#117a65" href="{base}/user/room">Správa miestnosti</a></li> |
	<li><a style="color:#117a65" href="{base}/user/one-time-activity">Správa jednorázových aktivít</a></li> |
	<li><a style="color:#117a65" href="{base}/user/repeated-activity">Správa opakovaných aktivít</a></li> |
	<li><a style="color:#117a65" href="{base}/user/upcoming-activities">Najbližšie aktivity</a></li> |
	<li><a style="color:#117a65" href="{base}/user/recent-activities">Nedávne aktivity</a></li> |
	<li><a style="color:#117a65" href="{base}/user/notifications">Notifikácie</a></li> |
	<li><a style="color:#117a65" href="{base}/user/settings">Nastavenia</a></li> |
	{/if}
	
	{#if auth.userState.role == AppUserRole.Unsigned}
		<li><a style="color:#000000" href="{base}/login">Prihlásiť sa</a></li>
	{:else}
		<li><a type="button" style="color:#000000" onclick={() => { auth.logout(); goto(base) }}>Odhlásiť sa</a></li>
	{/if}
</ul>

{@render children()}