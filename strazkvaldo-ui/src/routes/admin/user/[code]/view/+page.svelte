<script lang="ts">
	import { goto } from '$app/navigation';
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
	import {app_user_role_to_string} from '$lib/common';

	let { data }: PageProps = $props();
</script>

<h1>Používateľ {data.user.code}</h1>

<b>meno</b> {data.user.first_name}
<b>priezvisko</b> {data.user.last_name} <br>
<b>email</b> {data.user.email} <br>
<b>používateľské meno</b> {data.user.username} <br>
<b>rola</b> {app_user_role_to_string(data.user.app_user_role)}

<br>
<br>

<b>Vytvorený</b> {data.user.created} <br>
<b>Upravený</b> {data.user.updated} <br>
<button type="button" onclick={() => goto(`${base}/admin/user/${data.user.code}/edit`, { invalidateAll: true})}>Uprav použivateľa</button>