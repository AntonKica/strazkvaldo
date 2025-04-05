<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
    import { goto } from '$app/navigation';
    
	let { data }: PageProps = $props();
    console.log(data);
    
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const data = Object.fromEntries(formData);
        const formatted_data = {
		...data,
		app_user_role: Number(data.app_user_role)
        };
        
		fetch('/svc/app-user', {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formatted_data)
		}).then((response) => {
            if(response.ok) {
                goto(`${base}/admin/user/${response.headers.get('location')}/view`)
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
        <select name="app_user_role">
            <option value=0>administrátor</option>
            <option value=1>používateľ</option>
        </select>
	 </label> <br><br>
	<button type="submit">Vytvor použivateľa</button>
</form>