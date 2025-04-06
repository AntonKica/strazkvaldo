<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
	import {app_user_role_code_to_string, app_user_role_to_string} from '$lib/common';
	let { data }: PageProps = $props();
    console.log(data);
</script>

<h1>Používatelia</h1>

<a href="{base}/admin/user/create">Vytvor nového používateľa &#x2795</a>

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
        <td>{app_user_role_code_to_string(app_user.app_user_role)}</td>
        <td>{app_user.created}</td>
        <td>{app_user.updated}</td>
        <td><a href="{base}/admin/user/{app_user.code}/view">pozri</a></td>
    </tr>
    {/each}
    </tbody>
</table>