<script lang="ts">
	import { goto } from '$app/navigation';
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { SVC_ADMIN_APP_USER } from '$lib/serviceRoutes';

	let { data }: PageProps = $props();
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
		
		if (!formEntries.password?.trim()) {
			delete formEntries.password;
		}

        const formatted_data = {
		...formEntries
        };
        
		const res = await fetch(SVC_ADMIN_APP_USER.PATCH(data.user.code), {
			method: "PATCH",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		});

		if(!res.ok) {
			alert("Failed to update user")
			return;			
		}

		alert("Sucessfully updated user");
		goto_view();
	}
	
	const goto_view = () => {
		goto(`${base}/admin/user/${data.user.code}/view`, { invalidateAll: true});
	}
</script>

<h1>Používateľ {data.user.code}</h1>

<b>Vytvorený</b> {data.user.created} <br>
<b>Upravený</b> {data.user.updated} <br>
<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="first_name" type="text" value={data.user.first_name}> </label>
	<label> priezvisko <input name="last_name" type="text" value={data.user.last_name}> </label> <br>
	<label> email <input name="email" type="email" value={data.user.email}> </label> <br>
	<label> používateľské meno <input name="username" type="text" value={data.user.username}> </label>
	<label> heslo <input name="password" type="password"> </label> <br>
    <label> rola
        <select name="app_user_role" value={data.user.app_user_role.code}>
			{#each data.app_user_role as role}
				<option value={role.code}>{role.text}</option>
			{/each}
        </select>
	 </label> <br><br>
	<button type="submit">Ulož zmeny</button>
	<button type="button" onclick={goto_view}>Zahoď zmeny</button>
</form>