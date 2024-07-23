<script lang="ts">

</script>

<main id="secure-prompt">
    <div id="login-container">
        <h3>Login</h3>

        <div id="user-input">
            <div>
                <p>Email</p>
                <input type="email">
            </div>

            <div>
                <p>Password</p>
                <input type="password">
            </div>
        </div>

        <input id="login-button" type="button" value="login">
    </div>
</main>

<style>
    #secure-prompt {
        display: flex;
        flex-wrap: wrap;
        width: 100%;
        height: 100%;
        align-content: center;
        justify-content: center;
        user-select: none;

    }

    #login-container {
        display: flex;
        flex-direction: column;
        width: 300px;
        height: 400px;
        padding: 8px;
        border: #232323 1px solid;
    }

    #user-input input {
        outline: none;
        display: block;
        width: 100%;
        border: 1px solid #d9d9d9;
        margin: 0 0 20px;
        padding: 10px 15px;
        box-sizing: border-box;
        font-weight: 400;
        transition: 0.3s ease;
    }

    #user-input input:focus {
        border: 1px solid #00e600;
        color: #333333;
    }

    #login-button {
        cursor: pointer;
        width: 100%;
        padding: 10px 15px;
        background: #ffffff;
        border: 1px #d9d9d9 solid;
        transition: 0.3s ease;
    }

    #login-button:hover {
        border: 1px solid #00e600;
    }
</style>