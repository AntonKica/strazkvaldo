<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
    import { timestamp_to_string } from '$lib/common';
    import { UI_ADMIN_APP_USER } from '$lib/uiRoutes';
	let { data }: PageProps = $props();
    console.log(data);
</script>

<h1>Používatelia</h1>

<a href={UI_ADMIN_APP_USER.CREATE()}>Vytvor nového používateľa &#x2795</a>

<table>
    <thead>
        <tr>
            <td>kód použivateĺa</td>
            <td>používateľské meno</td>
            <td>rola</td>
            <td>vytvorený</td>
            <td>upravený</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
    {#each data.app_users as app_user}
    <tr>
        <td>{app_user.code}</td>
        <td>{app_user.username}</td>
        <td>{app_user.app_user_role.text}</td>
        <td>{timestamp_to_string(app_user.created)}</td>
        <td>{timestamp_to_string(app_user.updated)}</td>
        <td><a href={UI_ADMIN_APP_USER.VIEW(app_user.code)}>pozri</a></td>
    </tr>
    {/each}
    </tbody>
</table>