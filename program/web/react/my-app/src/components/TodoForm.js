import React from 'react'
import uuid from "uuid";

class TodoForm extends React.Component {

    styles = {
        'title': {
            width: 200,
            display: 'inline-block',
            marginRight: 10,
            verticalAlign: 'top'
        }
    }

    handleSubmit = (e) => {
        e.preventDefault()
        let text = this.refs.text.value
        if (!text.trim()) {
            console.log("Input can't be null")
            return
        }
        let id = uuid()
        this.props.AddTodoItem({ id, text, complete: false })
    }

    render() {
        return (
            <form class="ui reply form" onSubmit={this.handleSubmit}>
                <div class="field" style={this.styles.title} >
                    <input type="text" placeholder="TODO" ref="text" />
                </div>
                <button type="submit" class="ui submit button">添加</button>
            </form>
        )
    }
}

export default TodoForm
