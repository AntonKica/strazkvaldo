<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
    import { goto } from '$app/navigation';
    import { SVC_ADMIN_APP_USER } from '$lib/serviceRoutes';
    import { UI_ADMIN_APP_USER } from '$lib/uiRoutes';
    
	let { data }: PageProps = $props();
    let app_user_role = data.app_user_role;
    
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const data = Object.fromEntries(formData);
        const formatted_data = {
		...data,
        };
        
		fetch(SVC_ADMIN_APP_USER.POST(), {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		}).then((response) => {
            if(response.ok) {
                goto(UI_ADMIN_APP_USER.VIEW(response.headers.get('location') ?? ""));
            } else {
                alert("Failed to create user")
            }
        })
	}
</script>

<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="first_name" type="text"> </label>
	<label> priezvisko <input name="last_name" type="text"> </label> <br>
	<label> email <input name="email" type="email"> </label> <br>
	<label> používateľské meno <input name="username" type="text"> </label>
	<label> heslo <input name="password" type="password"> </label> <br>
    <label> rola
        <select name="app_user_role" value={app_user_role.code}>
			{#each data.app_user_role as role}
				<option value={role.code}>{role.text}</option>
			{/each}
        </select>
	 </label> <br><br>
	<button type="submit">Vytvor použivateľa</button>
</form>