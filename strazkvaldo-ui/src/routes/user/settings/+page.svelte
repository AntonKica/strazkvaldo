<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Nastavenia</h1>

<form>
    <label for="criticality">Úroveň kritickosti:</label><br>
    <input type="text" id="criticality" name="criticality" value="{data.settings.criticality}" readonly><br>
    <label for="lname">Notifikovať vopred dní:</label><br>
    <input type="text" id="lname" name="lname" value="{data.settings.notifyDaysBefore}" readonly> <br>
    <input type="submit" value="Ulož" disabled>
  </form> 