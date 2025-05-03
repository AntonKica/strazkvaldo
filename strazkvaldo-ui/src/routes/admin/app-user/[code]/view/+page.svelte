<script lang="ts">
	import { goto } from '$app/navigation';
    import { timestamp_to_string } from '$lib/common';
    import { SVC_ADMIN_APP_USER } from '$lib/serviceRoutes';
    import { delete_entity, UI_ADMIN_APP_USER } from '$lib/uiRoutes';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	
</script>

<h1>Používateľ {data.user.code}</h1>

<b>meno</b> {data.user.first_name}
<b>priezvisko</b> {data.user.last_name} <br>
<b>email</b> {data.user.email} <br>
<b>používateľské meno</b> {data.user.username} <br>
<b>rola</b> {data.user.app_user_role.text}

<br>
<br>

<b>Vytvorený</b> {timestamp_to_string(data.user.created)} <br>
<b>Upravený</b> {timestamp_to_string(data.user.updated)} <br>

<button type="button" onclick={() => goto(UI_ADMIN_APP_USER.EDIT(data.user.code), { invalidateAll: true})}>Uprav použivateľa</button> |
<button type="button" onclick={() => delete_entity(SVC_ADMIN_APP_USER.DELETE(data.user.code), UI_ADMIN_APP_USER.LIST())} style="color: red;">Vymaž používateľa</button>