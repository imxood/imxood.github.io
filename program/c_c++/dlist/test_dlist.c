#include <stdio.h>
#include "dlist.h"

/*

struct _dnode {
	union {
		struct _dnode *head;
		struct _dnode *next;
	};
	union {
		struct _dnode *tail;
		struct _dnode *prev;
	};
};

typedef struct _dnode sys_dlist_t;
typedef struct _dnode sys_dnode_t;

for sys_dlist_t, has member: head(首元素), tail(尾元素)
for sys_dlist_t, has member: next(前元素), prev(后元素)

ps:
	dlist保存的是首尾元素的指针, 若相等则dlist为空,

dlist init:
	dlist->head = head->tail = dlist

	ps: dlist的头尾相等

dlist append dnode:

	先调整dnode的前后元素:
		dnode->next = dlist;
		dnode->prev = dlist->tail;
	再调整dlist的尾元素:
		dlist->tail->next = dnode;
		dlist->tail = dnode;

	ps: dlist的尾节点改变

dlist preppend dnode:

	先调整dnode的前后元素:
		dnode->next = dlist->head;
		dnode->prev = dlist;
	再调整dlist的首元素:
		dlist->head->prev = dnode;
		dlist->head = dnode;

	ps: dlist的首节点改变

*/

int main(int argc, char const *argv[])
{
	sys_dlist_t list;

	sys_dlist_init(&list);

	sys_dnode_t node_1;
	sys_dnode_t node_2;
	sys_dnode_t node_3;
	sys_dnode_t node_4;

	sys_dlist_append(&list, &node_1);
	sys_dlist_append(&list, &node_2);
	sys_dlist_append(&list, &node_3);
	sys_dlist_append(&list, &node_4);

	sys_dnode_t *pnode;

	pnode = &list;

	printf("plist: %p\n", pnode);
	printf("\tphead: %p\n", pnode->head);
	printf("\tptail: %p\n", pnode->tail);
	printf("\tpnext: %p\n", pnode->next);
	printf("\tpprev: %p\n\n", pnode->prev);

	SYS_DLIST_FOR_EACH_NODE(&list, pnode) {
		printf("pnode: %p\n", pnode);
		printf("\tphead: %p\n", pnode->head);
		printf("\tptail: %p\n", pnode->tail);
		printf("\tpnext: %p\n", pnode->next);
		printf("\tpprev: %p\n", pnode->prev);
	}

	return 0;
}
