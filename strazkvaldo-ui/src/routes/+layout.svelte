<script>
    import { base } from '$app/paths';
	let { children } = $props();
</script>

<style>
	ul#menu li{
		display : inline;
	}
</style>


<ul id="menu">
	<li><a href="{base}/">Domov</a></li> |
	<li><a href="{base}/about">O aplikácií</a></li>
</ul>

{@render children()}