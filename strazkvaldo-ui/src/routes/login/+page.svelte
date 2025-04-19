<script lang="ts">
    import { goto } from '$app/navigation';
    import { base } from '$app/paths';
    import { app_user_role_from_int, AppUserRole } from '$lib/common';
	import { auth } from '$lib/userState.svelte';
    
	let username: string = "";
	let password: string = "";

    async function login(event: SubmitEvent) {
        event.preventDefault();
        
        const response = await fetch(`/svc/auth/login`, {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify({
                username: username,
                password: password
            })
		})
        

        if(response.status === 400) {
            const body = await response.json();
            alert("Failed to sign in: " + body.message);
            return;
        }
        if(response.status !== 200) {
            alert("Something went wrong, try again or not");
            return;
        }

        const body = await response.json();
        auth.login(username, body.code, app_user_role_from_int(body.role as number));
        goto(`${base}/`)
    }
</script>

<h1>Login</h1>


<form>
<label>Meno: <input type="text" bind:value={username}/></label><br>
<label>Heslo: <input type="password" bind:value={password}/></label> <br>

<button type="submit" onclick={login}>Prihlásiť sa</button>
</form>
