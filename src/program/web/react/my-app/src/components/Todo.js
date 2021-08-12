import React from 'react'


class Todo extends React.Component {

    styles = {
        'title': {
            paddingLeft: '20px',
            paddingRight: '50px',
            position: 'relative'
        },
        'delete': {
            marginLeft: '20px',
            marginRight: '50px'
        }
    }

    handleChangeComplete = () => {
        this.props.ChangeComplete(this.props.id)
    }

    handleDeleteItem = () => {
        this.props.DeleteItem(this.props.id)
    }

    render() {
        console.log(this.styles.title)
        return (
            <div class="comment">
                <div class="content">
                    <span class="author" style={this.styles.title}>
                        {this.props.text}
                        <span class={this.props.complete ? 'line' : ''}></span>
                    </span>
                    <span class="author" style={this.styles.title}>
                        {this.props.complete ? "已完成" : "未完成"}
                    </span>
                    <span class="author">{this.props.id}</span>
                    <span class="ui blue button" style={this.styles.delete}>删除</span>
                </div>
            </div>
        )
    }
}

export default Todo